import matplotlib.pyplot as plt
import metabodecon_python as md
import time


def main():
    blood = md.Spectrum.from_hdf5("../../metabodecon/data/blood.h5", "blood_01")
    deconvoluter = md.Deconvoluter(nfit = 10, sm_iter = 2, sm_ws = 5, delta = 6.4)
    t1 = time.time()
    deconvolution = deconvoluter.par_deconvolute_spectrum(blood)
    t2 = time.time()
    print(t2 - t1)
    plt.figure(figsize = (12, 8), dpi = 300)
    plt.plot(blood.chemical_shifts, deconvolution.par_superposition_vec(blood.chemical_shifts))
    plt.show()


if __name__ == "__main__":
    main()
