import numpy as np
import matplotlib.pyplot as plt
import metabodecon as md


def main():
    spectrum = md.Spectrum(
        chemical_shifts = np.array([1.0, 2.0, 3.0]),
        intensities = np.array([10.0, 20.0, 30.0]),
        signal_boundaries = (1.5, 2.5),
    )

    print(spectrum.chemical_shifts)
    spectrum.intensities = np.array([100.0, 200.0, 300.0])
    print(spectrum.intensities)
    print(spectrum.signal_boundaries)

    blood = md.Spectrum.read_hdf5("../../data/hdf5/blood.h5", "blood_01")
    plt.figure(figsize = (12, 8), dpi = 200)
    plt.plot(blood.chemical_shifts, blood.intensities)
    plt.show()
    plt.close()

    signal = (-2.208611, 11.807917)
    blood = md.Spectrum.read_bruker("../../data/bruker/blood/blood_01", 10, 10, signal)
    plt.figure(figsize = (12, 8), dpi = 200)
    plt.plot(blood.chemical_shifts, blood.intensities)
    plt.show()
    plt.close()

    signal = (3.339007, 3.553942)
    sim_set = md.Spectrum.read_bruker_set("../../data/bruker/sim", 10, 10, signal)
    for spectrum in sim_set[0:4]:
        plt.figure(figsize = (12, 8), dpi = 200)
        plt.plot(spectrum.chemical_shifts, spectrum.intensities)
        plt.show()
        plt.close()


if __name__ == "__main__":
    main()
