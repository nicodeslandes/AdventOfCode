import colorlog
import logging

_logging_initialised = False


def set_log_level(level):
    if not _logging_initialised:
        _initialise_logging()

    logging.root.setLevel(logging.DEBUG)


def _initialise_logging():
    # Rename log levels to 3-letter words
    logging.addLevelName(logging.DEBUG, 'DBG')
    logging.addLevelName(logging.INFO, 'INF')
    logging.addLevelName(logging.WARNING, 'WRN')
    logging.addLevelName(logging.ERROR, 'ERR')
    logging.addLevelName(logging.CRITICAL, 'CRT')

    # Fix color mapping to new names
    log_colors = {
        logging.getLevelName(logging.DEBUG): 'white',
        logging.getLevelName(logging.INFO): 'green',
        logging.getLevelName(logging.WARNING): 'yellow',
        logging.getLevelName(logging.ERROR): 'red',
        logging.getLevelName(logging.CRITICAL): 'bold_red',
    }

    # Setup color log handler
    handler = colorlog.StreamHandler()
    handler.setFormatter(colorlog.ColoredFormatter(
        '{log_color}{asctime} {levelname:.3} {message}',
        style='{', log_colors=log_colors))

    logging.root.addHandler(handler)
    _logging_initialised = True
