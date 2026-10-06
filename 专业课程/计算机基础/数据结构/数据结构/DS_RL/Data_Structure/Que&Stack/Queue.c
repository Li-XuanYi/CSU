#include <stdio.h>
#include <stdlib.h>
typedef struct ListNode
{
    int val;
    struct ListNode* next;
} ListNode;

typedef struct
{
   ListNode *front,*rear;
   int queSize;
} Queue;

// 构造函数
Queue *newQueue()
{
    Queue *queue = (Queue *)malloc(sizeof(Queue));
    queue->front = NULL;
    queue->rear = NULL; 
    queue->queSize = 0;
    return queue;
}

Queue *delQueue(Queue* queue)
{
    while(queue->front != NULL)
    {
        ListNode *tmp = queue->front;
        queue->front = tmp->next;
        free(tmp);
    }
    free(queue);
}
int peek(Queue* queue)
{
    return queue->front->val;
}
int pop(Queue* queue)
{
    int num = peek(queue);
    Queue* tmp = queue->front; 
    queue->front = tmp->front;
    free(tmp);
    queue->queSize--;
    return num;
}

// 自写 存在问题
// Queue* add(Queue* queue, int val)
// {
//     ListNode* q = (ListNode*)malloc(sizeof(ListNode));
//     q->val = val;
//     q->next = NULL;
//     queue->rear->next = q;
//     q = queue->rear;  // queue->rear = q;
//     queue->queSize++;
//     return queue;
// }

// Gemini Version
Queue* add(Queue* queue, int val)
{
    // 1. 容错处理：确保传入的 queue 不是 NULL
    if (queue == NULL) {
        return NULL; 
    }

    // 2. 分配新节点并检查内存是否分配成功
    ListNode* q = (ListNode*)malloc(sizeof(ListNode));
    if (q == NULL) {
        // 内存分配失败，可以打印错误日志或直接返回
        return queue; 
    }
    
    // 3. 初始化新节点
    q->val = val;
    q->next = NULL;

    // 4. 将新节点链入队列
    if (queue->rear == NULL) {
        // 情况 A：队列为空。新节点既是队头也是队尾
        queue->front = q;
        queue->rear = q;
    } else {
        // 情况 B：队列不为空。将新节点接到队尾后，并更新队尾指针
        queue->rear->next = q;
        queue->rear = q; // 修复了原代码写反的问题
    }

    // 5. 更新队列大小
    queue->queSize++;
    
    return queue;
}



int main()
{





    return 0;
}
