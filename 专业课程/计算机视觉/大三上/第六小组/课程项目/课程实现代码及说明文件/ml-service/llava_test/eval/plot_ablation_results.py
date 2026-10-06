import argparse
import os
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import json
import glob

def parse_args():
    parser = argparse.ArgumentParser(description="Plot Ablation Results from Partial CSVs")
    parser.add_argument("--output-dir", type=str, required=True, help="Directory containing summary_scores_part_*.csv files")
    return parser.parse_args()

def main():
    args = parse_args()
    
    # Find all partial CSV files
    csv_files = glob.glob(os.path.join(args.output_dir, "summary_scores_part_*.csv"))
    if not csv_files:
        print(f"No partial CSV files found in {args.output_dir}")
        return

    print(f"Found {len(csv_files)} partial CSV files. Merging...")
    
    # Read and merge
    dfs = []
    for f in csv_files:
        try:
            df = pd.read_csv(f)
            dfs.append(df)
        except Exception as e:
            print(f"Error reading {f}: {e}")
            
    if not dfs:
        print("No valid data found.")
        return
        
    full_df = pd.concat(dfs, ignore_index=True)
    
    # Sort by layer_idx
    full_df = full_df.sort_values("layer_idx")
    
    # Filter out baseline row if present (layer_idx = -1)
    # But we might want to keep it for reference, though plotting usually ignores it
    baseline_row = full_df[full_df['layer_idx'] == -1]
    ablation_df = full_df[full_df['layer_idx'] >= 0]
    
    # Save merged CSV
    merged_csv_path = os.path.join(args.output_dir, "summary_scores_merged.csv")
    full_df.to_csv(merged_csv_path, index=False)
    print(f"Merged data saved to {merged_csv_path}")
    
    # Visualization
    print("Visualizing results...")
    
    layers = ablation_df['layer_idx'].values
    scores = ablation_df['delta_CHAIRs'].values
    
    # Handle NaNs for plotting (maybe set to 0 or min value, or just leave as gap)
    # Matplotlib handles NaNs by leaving gaps, which is fine.
    
    plt.figure(figsize=(12, 6))
    plt.bar(layers, scores)
    plt.xlabel('Layer Index')
    plt.ylabel('Delta CHAIRs (Ablated - Baseline)')
    plt.title('Impact of MLP Zero-Ablation on Hallucination (CHAIRs) - N=500')
    plt.xticks(range(0, 32, 2)) # Adjust ticks as needed
    plt.grid(axis='y', linestyle='--', alpha=0.7)
    
    plot_path = os.path.join(args.output_dir, "ablation_results_merged.png")
    plt.savefig(plot_path)
    print(f"Plot saved to {plot_path}")
    
    # Top-5 Analysis
    # Filter NaNs
    valid_df = ablation_df.dropna(subset=['delta_CHAIRs'])
    
    if not valid_df.empty:
        top_5 = valid_df.nlargest(5, 'delta_CHAIRs')
        print("\nTop-5 Visual Critical Layers (Highest Delta CHAIRs):")
        print(top_5[['layer_idx', 'delta_CHAIRs']].to_string(index=False))
        
        top_5_list = top_5['layer_idx'].tolist()
    else:
        print("No valid scores found for ranking.")
        top_5_list = []
        
    # Save summary JSON
    baseline_val = baseline_row['CHAIRs'].iloc[0] if not baseline_row.empty else 0.0
    
    summary_json = {
        "baseline_chairs": baseline_val,
        "top_5": top_5_list,
        "layer_scores": ablation_df[['layer_idx', 'delta_CHAIRs']].to_dict('records')
    }
    
    json_path = os.path.join(args.output_dir, "scores_merged.json")
    with open(json_path, 'w') as f:
        json.dump(summary_json, f, indent=4)
    print(f"Summary JSON saved to {json_path}")

if __name__ == "__main__":
    main()
