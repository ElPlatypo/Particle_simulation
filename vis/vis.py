import pandas as pd
import matplotlib.pyplot as plt

# Read the CSV file
data = pd.read_csv('data.csv')

size = 30
# Extract the x and y values
xdata = data['x'].to_list()
ydata = data['y'].to_list()
print(xdata)
# Plot the data
plt.hexbin(xdata, ydata, gridsize=[30,15])
plt.xlabel('X')
plt.ylabel('Y')
plt.xlim([-1, size + 1])
plt.ylim([-1, size + 1])
plt.title("Repulsive particles grid")
plt.savefig("Grid.png")
