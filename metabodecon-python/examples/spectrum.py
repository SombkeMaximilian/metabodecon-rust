import numpy as np
import metabodecon_python as md


def main():
    spectrum = md.MdSpectrum(
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

    blood = md.MdSpectrum.from_hdf5("../../metabodecon/data/blood.h5", "blood_01")
    print(blood.chemical_shifts[:5])
    print(blood.intensities[:5])
    print(blood.intensities_raw[:5])
    print(blood.signal_boundaries)
    print(blood.water_boundaries)


if __name__ == "__main__":
    main()
