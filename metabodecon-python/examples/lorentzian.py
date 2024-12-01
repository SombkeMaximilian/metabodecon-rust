import numpy as np
import matplotlib.pyplot as plt
import metabodecon_python as md


def main():
    lorentzian = md.Lorentzian(
        sf = 1.0,
        hw = 1.0,
        maxp = 0.0
    )

    print(lorentzian.sf)
    print(lorentzian.hw)
    print(lorentzian.maxp)
    lorentzian.sf = 2.0
    lorentzian.hw = 1.5
    print(lorentzian.sf)
    print(lorentzian.hw)
    print(lorentzian.maxp)

    lorentzian.parameters = (3.0, 2.0, 0.0)
    x = np.linspace(-10, 10, 10000)
    y = lorentzian.evaluate_vec(x)
    plt.plot(x, y)
    plt.show()

    sf = [1.0, 2.0, 1.0]
    hw = [0.1, 0.15, 0.1]
    maxp = [4.5, 5.0, 5.5]
    lorentzians = [md.Lorentzian(sf = sf[i], hw = hw[i], maxp = maxp[i]) for i in range(3)]
    x = np.linspace(0, 10, 100000)
    y = md.Lorentzian.par_superposition_vec(x, lorentzians)
    plt.plot(x, y)
    plt.show()


if __name__ == "__main__":
    main()
