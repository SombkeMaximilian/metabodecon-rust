import numpy as np

__version__: str


class Deconvoluter:
    def __init__(self) -> None:
        ...

    def with_moving_average_smoother(self, iterations: int, window_size: int) -> "Deconvoluter":
        ...

    def with_noise_score_selector(self, threshold: float) -> "Deconvoluter":
        ...

    def with_analytical_fitter(self, iterations: int) -> "Deconvoluter":
        ...

    def deconvolute_spectrum(self, spectrum: "Spectrum") -> "Deconvolution":
        ...

    def par_deconvolute_spectrum(self, spectrum: "Spectrum") -> "Deconvolution":
        ...


class Deconvolution:
    lorentzians: list["Lorentzian"]
    mse: float

    def superposition_vec(self, x: np.ndarray) -> np.ndarray:
        ...

    def par_superposition_vec(self, x: np.ndarray) -> np.ndarray:
        ...


class Lorentzian:
    sf: float
    hw: float
    maxp: float

    def __init__(self, sf: float, hw: float, maxp: float) -> None:
        ...

    def evaluate(self, x: float) -> float:
        ...

    def evaluate_vec(self, x: np.ndarray) -> np.ndarray:
        ...

    @staticmethod
    def superposition(x: float, lorentzians: list["Lorentzian"]) -> float:
        ...

    @staticmethod
    def superposition_vec(x: np.ndarray, lorentzians: list["Lorentzian"]) -> np.ndarray:
        ...

    @staticmethod
    def par_superposition_vec(x: np.ndarray, lorentzians: list["Lorentzian"]) -> np.ndarray:
        ...


class Spectrum:
    chemical_shifts: np.ndarray
    intensities_raw: np.ndarray
    intensities: np.ndarray
    signal_boundaries: tuple[float, float]
    water_boundaries: tuple[float, float]

    def __init__(self, chemical_shifts: np.ndarray, intensities: np.ndarray, signal_boundaries: tuple[float, float],
                 water_boundaries: tuple[float, float]) -> None:
        ...

    @staticmethod
    def from_bruker(path: str, experiment: int, processing: int, signal_boundaries: tuple[float, float],
                    water_boundaries: tuple[float, float]) -> "Spectrum":
        ...

    @staticmethod
    def from_bruker_set(path: str, experiment: int, processing: int, signal_boundaries: tuple[float, float],
                        water_boundaries: tuple[float, float]) -> list["Spectrum"]:
        ...

    @staticmethod
    def from_hdf5(path: str, dataset: str) -> "Spectrum":
        ...

    @staticmethod
    def from_hdf5_set(path: str) -> list["Spectrum"]:
        ...
