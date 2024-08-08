import argparse
import json
import matplotlib.pyplot as plt
import os
import pandas as pd

def load_data_from_files(directory, mode):
    for filename in os.listdir(directory):
        if filename.endswith('.json'):
            with open(os.path.join(directory, filename), 'r') as file:
                create_display(json.load(file), filename, mode)

def create_display(data, filename, mode):
    match mode:
        case 'mean':
            title = "Mean"
            unit = "(ms)"
        case 'median':
            title = "Median"
            unit = "(ms)"
        case 'total':
            title = "Total"
            unit = "(s)"
        case _:
            print(f"Invalid mode selected: {mode}")
    df = pd.DataFrame(data)
    index_name = f"{mode}"
    filtered_df = df[df[index_name] > 500]
    use_chain_as_label = 'chain' in filtered_df.columns

    # Output directories
    output_dir = f"output/bar_charts/{index_name}"

    # Ensure the directory exists
    if not os.path.exists(output_dir):
        os.makedirs(output_dir)

    y_label = f'{title} Runtime {unit}'

    # Plot
    if use_chain_as_label:
        chains = filtered_df['chain'].unique()
        for chain in chains:
            chain_data = filtered_df[filtered_df['chain'] == chain]
            plt.figure(figsize=(14, 7))
            plt.bar(chain_data['name'], chain_data[index_name], color='skyblue')
            plt.title(f'{title} Runtime')
            plt.xlabel('Query Name')
            plt.ylabel(y_label)
            plt.xticks(rotation=90)
            plt.grid(axis='y')
            plt.tight_layout()
            plt.savefig(f"{output_dir}/{chain}_{filename}.png")
    else:
        plt.figure(figsize=(14, 7))
        plt.bar(filtered_df['name'], filtered_df[index_name], color='skyblue')
        plt.title(f'{title} Runtime')
        plt.xlabel('Query Name')
        plt.ylabel(y_label)
        plt.xticks(rotation=90)
        plt.grid(axis='y')
        plt.tight_layout()
        plt.savefig(f"{output_dir}/{filename}.png")

parser = argparse.ArgumentParser(description="Create Box Plot of parsed data")

parser.add_argument('directory', type=str, help="The directory containing the .json files")
parser.add_argument(
        '--mode',
        type=str,
        choices=['mean', 'median', 'total'],
        required=True,
        help="Specify the metric to use: 'mean', 'median' or 'total'."
    )

args = parser.parse_args()

load_data_from_files(args.directory, args.mode)