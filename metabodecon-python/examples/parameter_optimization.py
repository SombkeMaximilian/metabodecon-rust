import itertools
import time
import matplotlib.pyplot as plt
import metabodecon as md
import numpy as np
import pandas as pd

def main():
    smoothing_iterations = [2, 3, 4, 5, 6, 7, 10, 12, 15, 20]
    smoothing_window_sizes = [3, 5, 7, 9, 11, 13, 15, 17, 19, 21]
    noise_score_thresholds = np.linspace(start = 5, stop = 8, num = 10).tolist()
    fitter_iterations = [1, 2, 3, 4, 5, 7, 10, 12, 15, 20]

    combinations = list(
        itertools.product(
            smoothing_iterations,
            smoothing_window_sizes,
            noise_score_thresholds,
            fitter_iterations
        )
    )
    param_table = pd.DataFrame(
        data = combinations,
        columns = [
            "Smoothing Iterations",
            "Smoothing Window Size",
            "Noise Score Threshold",
            "Fitter Iterations",
        ]
    )
    param_table["MSE"] = np.nan

    spectra = md.Spectrum.from_bruker_set("../../data/bruker/sim", 10, 10, (3.33, 3.56), (3.445, 3.448))
    deconvoluter = md.Deconvoluter()
    optimal_parameters = []

    for i, spectrum in enumerate(spectra):
        t1 = time.time()
        for index, row in param_table.iterrows():
            deconvoluter.with_moving_average_smoother(
                int(row["Smoothing Iterations"]),
                int(row["Smoothing Window Size"])
            )
            deconvoluter.with_noise_score_selector(
                row["Noise Score Threshold"]
            )
            deconvoluter.with_analytical_fitter(
                int(row["Fitter Iterations"])
            )
            param_table.at[index, "MSE"] = deconvoluter.deconvolute_spectrum(spectrum).mse

        t2 = time.time()
        print(f"Elapsed time {(t2 - t1) * 1000:.3f}ms")

        min_mse_per_parameter = {
            "Smoothing Iterations": param_table.groupby("Smoothing Iterations")["MSE"].min(),
            "Smoothing Window Size": param_table.groupby("Smoothing Window Size")["MSE"].min(),
            "Noise Score Threshold": param_table.groupby("Noise Score Threshold")["MSE"].min(),
            "Fitter Iterations": param_table.groupby("Fitter Iterations")["MSE"].min()
        }

        fig, axs = plt.subplots(2, 2, figsize = (12, 10), dpi = 300)
        fig.suptitle(f"Minimum MSE by parameter for spectrum {(i + 1):02}", size = 20)

        for ax, (key, value) in zip(axs.flatten(), min_mse_per_parameter.items()):
            ax.plot(value, marker = "o")
            ax.set_title(key)
            ax.set_xlabel(key)
            ax.set_ylabel("MSE")

        plt.tight_layout(rect = (0.0, 0.0, 1.0, 0.96))
        plt.show()

        optimal_parameters.append(param_table.loc[param_table["MSE"].idxmin()])

    for p in optimal_parameters:
        print(p)


if __name__ == "__main__":
    main()
