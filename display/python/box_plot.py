import argparse
import json
import matplotlib.pyplot as plt
import os
import pandas as pd
import plotly.express as px

def load_data_from_files(directory):
    for filename in os.listdir(directory):
        if filename.endswith('.json'):
            with open(os.path.join(directory, filename), 'r') as file:
                create_display(json.load(file), filename)

def create_display(data, filename):
    df = pd.DataFrame(data)
    names = df['name'].unique()
    use_chain_as_label = 'chain' in df.columns

    for name in names:
        name_data = df[df['name'] == name]
    
        # Prepare data for boxplot
        box_data = []
        labels = []
        for _, row in name_data.iterrows():
            box_data.append([row['min'], row['q25'], row['median'], row['q75'], row['max']])
            label = row['chain'] if use_chain_as_label else row['name']
            labels.append(label)
        
            # Plot
            plt.figure(figsize=(12, 6))
            plt.boxplot(box_data, tick_labels=labels)
            plt.title(f'Runtime Statistics for {row["count"]} calls to {name}')
            plt.xlabel('Chain')
            plt.ylabel('Time (ms)')
            plt.xticks(rotation=45)  # Rotate labels for better readability
            plt.tight_layout()  # Adjust layout for better fit
            plt.savefig(f"output/{filename}_{name}.png")

parser = argparse.ArgumentParser(description="Create Box Plot of parsed data")

parser.add_argument('directory', type=str, help="The directory containing the .json files")

args = parser.parse_args()

load_data_from_files(args.directory)