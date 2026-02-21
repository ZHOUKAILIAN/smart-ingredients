import sys
import unittest
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[1]))

from ocr_utils import parse_ocr_result


class TestParseOcrResult(unittest.TestCase):
    def test_none_result_returns_empty(self):
        text, lines = parse_ocr_result([None], min_text_len=2)
        self.assertEqual(text, "")
        self.assertEqual(lines, [])

    def test_empty_list_returns_empty(self):
        text, lines = parse_ocr_result([], min_text_len=2)
        self.assertEqual(text, "")
        self.assertEqual(lines, [])

    def test_collects_text_and_lines(self):
        sample = [
            [
                [[[0, 0], [1, 0], [1, 1], [0, 1]], ("Ingredients: Water", 0.98)],
                [[[0, 0], [1, 0], [1, 1], [0, 1]], ("Sugar", 0.95)],
            ]
        ]
        text, lines = parse_ocr_result(sample, min_text_len=2)
        self.assertEqual(text, "Ingredients: Water\nSugar")
        self.assertEqual(len(lines), 2)
        self.assertEqual(lines[0]["text"], "Ingredients: Water")


if __name__ == "__main__":
    unittest.main()
