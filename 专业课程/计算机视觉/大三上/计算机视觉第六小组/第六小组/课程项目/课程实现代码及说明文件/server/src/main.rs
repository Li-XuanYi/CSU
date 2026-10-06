use mindsight_gallery_server::handlers::{albums, docs, health, photos, processing, users};
use mindsight_gallery_server::middleware::auth::{admin_only, jwt_auth};
use mindsight_gallery_server::{Config, db};
use redis::Client;
use salvo::affix_state;
use salvo::http::Method;
use salvo::http::header::HeaderValue;
use salvo::oapi::security::{Http, HttpAuthScheme};
use salvo::oapi::swagger_ui::SwaggerUi;
use salvo::oapi::{OpenApi, SecurityScheme};
use salvo::prelude::*;
use salvo::serve_static::StaticDir;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::Mutex;

/// Minimal CORS handler to temporarily allow cross-origin calls.
#[handler]
async fn cors(req: &mut Request, res: &mut Response, ctrl: &mut FlowCtrl) {
    // Echo origin if present, otherwise allow all (temporary).
    let origin = req
        .headers()
        .get("Origin")
        .cloned()
        .unwrap_or_else(|| HeaderValue::from_static("*"));

    res.headers_mut()
        .insert("Access-Control-Allow-Origin", origin);
    res.headers_mut().insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("authorization, content-type, x-internal-token"),
    );
    res.headers_mut().insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    res.headers_mut()
        .insert("Access-Control-Max-Age", HeaderValue::from_static("86400"));

    // Short-circuit preflight requests
    if req.method() == Method::OPTIONS {
        res.status_code(StatusCode::NO_CONTENT);
        ctrl.skip_rest();
    }
}

/// Catch-all OPTIONS so preflight always succeeds.
#[handler]
async fn cors_preflight(req: &mut Request, res: &mut Response) {
    let origin = req
        .headers()
        .get("Origin")
        .cloned()
        .unwrap_or_else(|| HeaderValue::from_static("*"));

    res.headers_mut()
        .insert("Access-Control-Allow-Origin", origin);
    res.headers_mut().insert(
        "Access-Control-Allow-Headers",
        HeaderValue::from_static("authorization, content-type, x-internal-token"),
    );
    res.headers_mut().insert(
        "Access-Control-Allow-Methods",
        HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
    );
    res.headers_mut()
        .insert("Access-Control-Max-Age", HeaderValue::from_static("86400"));

    res.status_code(StatusCode::NO_CONTENT);
}

#[tokio::main]
async fn main() {
    // Initialize logging subsystem
    tracing_subscriber::fmt().init();

    // Load configuration from environment
    let config = Config::from_env().expect("Failed to load configuration");
    tracing::info!("Configuration loaded successfully");

    // Initialize database connection pool
    let db_pool = db::init_db_pool(&config.database)
        .await
        .expect("Failed to initialize database connection pool");

    // Initialize Redis connection manager for queueing tasks
    let redis_client =
        Client::open(config.redis.url.as_str()).expect("Failed to create Redis client");
    let redis_manager = redis_client
        .get_connection_manager()
        .await
        .expect("Failed to connect to Redis");

    // 将共享资源包装在Arc中，避免每次请求都clone
    // Arc (Atomic Reference Counted) 提供线程安全的共享访问
    let db_pool = Arc::new(db_pool);
    let redis_manager = Arc::new(Mutex::new(redis_manager));
    let config = Arc::new(config);
    let face_dirty = Arc::new(AtomicBool::new(false));

    // 后台任务：每3分钟检查一次是否有新的人脸写入并生成相册
    let db_for_job = db_pool.clone();
    let config_for_job = config.clone();
    let dirty_for_job = face_dirty.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(180));
        loop {
            interval.tick().await;
            if !dirty_for_job.swap(false, Ordering::SeqCst) {
                continue;
            }
            if let Err(e) =
                photos::run_auto_people_job(db_for_job.as_ref(), config_for_job.as_ref()).await
            {
                tracing::error!(error = %e, "Auto people album job failed");
                dirty_for_job.store(true, Ordering::SeqCst);
            }
        }
    });

    // 后台任务：定时补齐 ML 预处理任务
    let db_for_ml = db_pool.clone();
    let redis_for_ml = redis_manager.clone();
    let config_for_ml = config.clone();
    tokio::spawn(async move {
        let mut interval =
            tokio::time::interval(Duration::from_secs(config_for_ml.processing.planner_interval_secs));
        loop {
            interval.tick().await;
            if let Err(e) =
                processing::run_ml_planner_job(db_for_ml.as_ref(), &redis_for_ml, config_for_ml.as_ref()).await
            {
                tracing::error!(error = %e, "ML planner job failed");
            }
        }
    });

    // Public routes (no authentication required)
    let public_routes = Router::new()
        .push(Router::with_path("register").post(users::register))
        .push(Router::with_path("login").post(users::login))
        .push(Router::with_path("refresh").post(users::refresh_token));

    // Protected routes (authentication required)
    let protected_routes = Router::new()
        .hoop(jwt_auth)
        .push(Router::with_path("users").get(users::list_users))
        .push(Router::with_path("processing/progress").get(processing::get_processing_progress))
        .push(Router::with_path("processing/control").post(processing::update_processing_control))
        // Album routes
        .push(Router::with_path("albums").get(albums::list_albums))
        .push(Router::with_path("albums").post(albums::create_album))
        .push(Router::with_path("albums/{id}").get(albums::get_album))
        .push(Router::with_path("albums/{id}").put(albums::update_album))
        .push(
            Router::with_path("albums/{id}/photos/{photo_id}")
                .post(albums::add_album_photo)
                .delete(albums::remove_album_photo),
        )
        .push(Router::with_path("albums/{id}").delete(albums::delete_album))
        // Photo routes
        .push(Router::with_path("photos/upload").post(photos::upload_photo))
        .push(Router::with_path("photos").get(photos::list_my_photos))
        // 兼容正确的路径占位符写法，避免 404
        .push(Router::with_path("photos/album/{album_id}").get(photos::list_photos))
        .push(
            Router::with_path("photos/{photo_id}/albums/{album_id}")
                .post(photos::add_photo_to_album),
        )
        .push(
            Router::with_path("photos/{photo_id}/albums/{album_id}")
                .delete(photos::remove_photo_from_album),
        )
        .push(Router::with_path("photos/{id}").get(photos::get_photo))
        .push(Router::with_path("photos/{id}").put(photos::update_photo_metadata))
        .push(Router::with_path("photos/{id}").delete(photos::delete_photo))
        .push(Router::with_path("photos/{id}/similar").get(photos::similar_photos))
        .push(Router::with_path("photos/{id}/llava").post(photos::create_llava_conversation))
        .push(
            Router::with_path("photos/{id}/llava/{conversation_id}")
                .post(photos::continue_llava_conversation)
                .get(photos::get_llava_conversation),
        )
        .push(Router::with_path("search/vector").post(photos::search_by_vector))
        .push(Router::with_path("search/fusion").post(photos::search_by_fusion))
        .push(Router::with_path("search/text").post(photos::search_by_text))
        .push(Router::with_path("search/text/{id}").get(photos::get_text_search_status))
        .push(Router::with_path("photos/auto_people_albums").post(photos::auto_people_albums));

    // Admin routes (authentication + admin role required)
    let admin_routes = Router::new()
        .hoop(jwt_auth)
        .hoop(admin_only)
        .push(Router::with_path("users").post(users::create_user));

    let docs_routes = Router::new().push(docs::scalar_router());

    let internal_routes = Router::new()
        .push(Router::with_path("photos/{id}/ml_result").post(photos::update_ml_result))
        .push(Router::with_path("photos/{id}/faces").post(photos::update_faces))
        .push(Router::with_path("text_queries/{id}").post(photos::update_text_query))
        .push(
            Router::with_path("photo_conversations/{id}")
                .post(photos::update_llava_conversation),
        );

    // All API routes share the /api prefix so docs match real paths
    let api_router = Router::new()
        .push(Router::with_path("api/auth").push(public_routes))
        .push(Router::with_path("api").push(protected_routes))
        .push(Router::with_path("api/admin").push(admin_routes))
        .push(Router::with_path("api/internal").push(internal_routes));

    // Configure OpenAPI with prefixed routes
    let doc = OpenApi::new("Mindsight Gallery API", "0.1.0")
        .add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                Http::new(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .description("JWT Authorization header using the Bearer scheme"),
            ),
        )
        .merge_router(&api_router);

    // Build router with all endpoints
    let router = Router::new()
        .hoop(cors)
        .hoop(affix_state::inject(db_pool.clone()))
        .hoop(affix_state::inject(redis_manager.clone()))
        .hoop(affix_state::inject(config.clone()))
        .hoop(affix_state::inject(face_dirty.clone()))
        // Static file serving for uploads - put first to avoid conflicts
        .push(Router::with_path("uploads/{*path}").get(StaticDir::new(["uploads"])))
        .push(Router::with_path("health").get(health::health_check))
        .push(api_router)
        .push(Router::with_path("docs").push(docs_routes))
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("/swagger-ui"))
        // Catch-all OPTIONS to ensure CORS preflight never 404s
        .push(Router::with_path("<**any>").options(cors_preflight));

    // Bind server to configured host and port
    let bind_addr = format!("{}:{}", config.server.host, config.server.port);
    tracing::info!("Starting server on {}", bind_addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui", bind_addr);

    let acceptor = TcpListener::new(&bind_addr).bind().await;

    // Start serving requests
    Server::new(acceptor).serve(router).await;
}
