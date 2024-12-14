import matplotlib.pyplot as plt
import metabodecon as md
import time


def main():
    blood = md.Spectrum.from_hdf5("../../data/hdf5/blood.h5", "blood_01")
    blood.signal_boundaries = (-2.208611, 11.807918)
    blood.water_boundaries = (4.699535, 4.899771)

    deconvoluter = (md.Deconvoluter()
                        .with_ma_smoother(2, 5)
                        .with_def_selector(6.4)
                        .with_analytical_fitter(10))
    t1 = time.time()
    deconvolution = deconvoluter.deconvolute_spectrum(blood)
    t2 = time.time()
    print(f"Sequential {(t2 - t1) * 1000:.3f}ms")
    t1 = time.time()
    deconvolution = deconvoluter.par_deconvolute_spectrum(blood)
    t2 = time.time()
    print(f"Parallel {(t2 - t1) * 1000:.3f}ms")

    x = blood.chemical_shifts
    y = deconvolution.par_superposition_vec(blood.chemical_shifts)
    s = blood.signal_boundaries
    w = blood.water_boundaries
    plt.figure(figsize = (12, 8), dpi = 300)
    plt.plot(x, y, label = "Deconvoluted Spectrum")
    plt.gca().invert_xaxis()
    plt.axvline(x = s[0], color = "black", label = "Signal Boundaries")
    plt.axvline(x = s[1], color = "black")
    plt.axvspan(w[0], w[1], color = "cyan", alpha = 0.3, label = "Water Region")
    plt.xlabel("Chemical Shifts", fontsize = 16)
    plt.ylabel("Intensity", fontsize = 16)
    plt.xticks(fontsize = 14)
    plt.yticks(fontsize = 14)
    plt.legend()
    plt.show()


if __name__ == "__main__":
    main()
