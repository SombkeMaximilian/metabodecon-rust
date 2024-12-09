import numpy as np
import matplotlib.pyplot as plt
import metabodecon_python as md


def main():
    spectrum = md.Spectrum(
        chemical_shifts = np.array([1.0, 2.0, 3.0]),
        intensities = np.array([10.0, 20.0, 30.0]),
        signal_boundaries = (1.5, 2.5),
        water_boundaries = (1.75, 2.25)
    )

    print(spectrum.chemical_shifts)
    spectrum.intensities = np.array([100.0, 200.0, 300.0])
    print(spectrum.intensities)
    print(spectrum.intensities_raw)
    print(spectrum.signal_boundaries)
    print(spectrum.water_boundaries)

    blood = md.Spectrum.from_hdf5("../../data/hdf5/blood.h5", "blood_01")
    plt.figure(figsize = (12, 8), dpi = 300)
    plt.plot(blood.chemical_shifts, blood.intensities_raw)
    plt.show()

    sim = md.Spectrum.from_bruker("../../data/bruker/sim/sim_01", 10, 10, (3.339007, 3.553942), (3.444939, 3.448010))
    plt.figure(figsize = (12, 8), dpi = 300)
    plt.plot(sim.chemical_shifts, sim.intensities_raw)
    plt.show()


if __name__ == "__main__":
    main()
