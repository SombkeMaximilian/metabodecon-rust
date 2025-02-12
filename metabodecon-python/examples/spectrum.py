import matplotlib.pyplot as plt
import metabodecon as md


def main():
    signal = (-2.208611, 11.807917)
    blood = md.Spectrum.read_bruker("../../data/bruker/blood/blood_01", 10, 10, signal)
    plt.figure(figsize = (12, 8), dpi = 200)
    plt.plot(blood.chemical_shifts, blood.intensities)
    plt.show()
    plt.close()

    blood.write_json("blood.json")
    blood_json = md.Spectrum.read_json("blood.json")
    plt.figure(figsize = (12, 8), dpi = 200)
    plt.plot(blood_json.chemical_shifts, blood_json.intensities)
    plt.show()
    plt.close()

    blood.write_bin("blood.bin")
    blood_bin = md.Spectrum.read_bin("blood.bin")
    plt.figure(figsize = (12, 8), dpi = 200)
    plt.plot(blood_bin.chemical_shifts, blood_bin.intensities)
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
