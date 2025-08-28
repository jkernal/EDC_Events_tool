import logging
import sys

_LEVELS = {
    "CRITICAL": logging.CRITICAL,
    "ERROR": logging.ERROR,
    "WARNING": logging.WARNING,
    "INFO": logging.INFO,
    "DEBUG": logging.DEBUG
}

FORMAT = '%(asctime)s | %(levelname)s | %(message)s'

def setup_logging(logger_name, level, filename=__name__, log_dest="file", format=FORMAT):
    
    logger = logging.getLogger(logger_name)
    logger.setLevel(_LEVELS.get(level))
    
    formatter = logging.Formatter(format)
    
    if log_dest == "file" or log_dest == "both":
        file_handler = logging.FileHandler(filename, encoding="utf-8")
        file_handler.setLevel(_LEVELS.get(level))
        file_handler.setFormatter(formatter)
        logger.addHandler(file_handler)
    if log_dest == "terminal" or log_dest == "both":
        terminal_handler = logging.StreamHandler(sys.stdout)
        terminal_handler.setLevel(_LEVELS.get(level))
        terminal_handler.setFormatter(formatter)
        logger.addHandler(terminal_handler)
    
    return logger
        
