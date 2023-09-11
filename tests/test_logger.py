import glob
import tempfile


def test_info_logger():
    from rusty_logger import JsonLogger

    with tempfile.TemporaryDirectory() as tmp_dir:
        logger = JsonLogger.get_logger(name=__file__, output=f"test.log", level="DEBUG")
        logger.info("test info")
        logger.debug("test debug")
        logger.warning("test warning")
        logger.error("test error")

        assert glob.glob(f"logs/test.log*")

        for name in glob.glob(f"logs/test.log*"):
            print(name)
            with open(name, "r") as fp:
                for count, line in enumerate(fp):
                    pass
                count = count + 1

        assert count == 3
