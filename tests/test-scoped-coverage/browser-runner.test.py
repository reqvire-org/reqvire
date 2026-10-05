import importlib.util
from pathlib import Path
import sys
import tempfile
import unittest

spec = importlib.util.spec_from_file_location(
    "scoped_coverage", Path(__file__).with_name("check_scoped_coverage.py"))
coverage = importlib.util.module_from_spec(spec)
spec.loader.exec_module(coverage)


class BrowserDeadlineTest(unittest.TestCase):
    def test_timeout_retains_partial_diagnostics(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            command = [sys.executable, "-u", "-c",
                       "import sys,time; print('first: PASS'); "
                       "print('START reload', file=sys.stderr); time.sleep(30)"]
            with self.assertRaisesRegex(AssertionError, "(?s)timed out.*START reload"):
                coverage.run_browser(command, output, timeout=0.5)
            self.assertEqual((output / "browser.stdout").read_text(), "first: PASS\n")
            self.assertEqual((output / "browser.stderr").read_text(), "START reload\n")

    def test_failure_and_success_keep_logs(self):
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            with self.assertRaisesRegex(AssertionError, "(?s)Browser failed:.*failed assertion"):
                coverage.run_browser([sys.executable, "-c",
                    "import sys; print('failed assertion', file=sys.stderr); sys.exit(1)"], output)
            self.assertEqual(coverage.run_browser([sys.executable, "-c", "print('PASS')"], output), "PASS\n")


if __name__ == "__main__":
    unittest.main()
