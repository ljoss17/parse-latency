import argparse
import json
import matplotlib.pyplot as plt
import os
import pandas as pd

def load_data_from_files(directory):
    for filename in os.listdir(directory):
        if filename.endswith('.json'):
            with open(os.path.join(directory, filename), 'r') as file:
                create_display(json.load(file), filename)

def create_display(data, filename):
    df = pd.DataFrame(data)
    filtered_df = df[df['mean'] > 500]
    use_chain_as_label = 'chain' in filtered_df.columns

    # Output directories
    output_dir = "output/bar_charts"

    # Ensure the directory exists
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    # Plot
    if use_chain_as_label:
        chains = filtered_df['chain'].unique()
        for chain in chains:
            chain_data = filtered_df[filtered_df['chain'] == chain]
            plt.figure(figsize=(14, 7))
            plt.bar(chain_data['name'], chain_data['mean'], color='skyblue')
            plt.title(f'Mean Runtime')
            plt.xlabel('Query Name')
            plt.ylabel('Mean Runtime (ms)')
            plt.xticks(rotation=45)
            plt.grid(axis='y')
            plt.tight_layout()
            plt.savefig(f"{output_dir}/{chain}_{filename}.png")
    else:
        plt.figure(figsize=(14, 7))
        plt.bar(filtered_df['name'], filtered_df['mean'], color='skyblue')
        plt.title(f'Mean Runtime')
        plt.xlabel('Query Name')
        plt.ylabel('Mean Runtime (ms)')
        plt.xticks(rotation=45)
        plt.grid(axis='y')
        plt.tight_layout()
        plt.savefig(f"{output_dir}/{filename}.png")

parser = argparse.ArgumentParser(description="Create Box Plot of parsed data")

parser.add_argument('directory', type=str, help="The directory containing the .json files")

args = parser.parse_args()

load_data_from_files(args.directory)