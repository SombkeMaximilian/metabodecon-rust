class Error(Exception):
    """Base class for exceptions in this module."""

    pass

class Unexpected(Error):
    """An exception raised for unexpected errors."""

    pass

class SerializationError(Error):
    """An exception raised for errors in serialization."""

    pass

class SpectrumError(Error):
    """An exception raised for errors in the Spectrum class."""

    pass

class EmptyData(SpectrumError):
    """Input data is empty."""

    pass

class DataLengthMismatch(SpectrumError):
    """Input data lengths do not match."""

    pass

class NonUniformSpacing(SpectrumError):
    """Chemical shifts are not uniformly spaced."""

    pass

class InvalidIntensities(SpectrumError):
    """Intensities contain invalid values."""

    pass

class InvalidSignalBoundaries(SpectrumError):
    """Signal boundaries are invalid."""

    pass

class MissingMetadata(SpectrumError):
    """Metadata is missing from NMR format-related file."""

    pass

class MalformedMetadata(SpectrumError):
    """Metadata in some NMR format-related file is malformed."""

    pass

class MissingData(SpectrumError):
    """Some NMR format-related file contains no data."""

    pass

class MalformedData(SpectrumError):
    """Data in some NMR format-related file is malformed."""

    pass

class DeconvolutionError(Error):
    """An exception raised for errors during the deconvolution process."""

    pass

class InvalidSmoothingSettings(DeconvolutionError):
    """Smoothing settings are invalid."""

    pass

class InvalidSelectionSettings(DeconvolutionError):
    """Peak selection settings are invalid."""

    pass

class InvalidFittingSettings(DeconvolutionError):
    """Peak fitting settings are invalid."""

    pass

class InvalidIgnoreRegion(DeconvolutionError):
    """Ignore region boundaries are invalid."""

    pass

class NoPeaksDetected(DeconvolutionError):
    """No peaks were detected in the spectrum."""

    pass

class EmptySignalRegion(DeconvolutionError):
    """Signal region contains no peaks."""

    pass

class EmptySignalFreeRegion(DeconvolutionError):
    """Signal-free region contains no peaks."""

    pass
