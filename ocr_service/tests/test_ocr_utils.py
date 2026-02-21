import sys
import unittest
from pathlib import Path

sys.path.append(str(Path(__file__).resolve().parents[1]))

from ocr_utils import build_ocr_response, parse_ocr_result


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


class TestBuildOcrResponse(unittest.TestCase):
    def test_empty_result_returns_422(self):
        status, payload = build_ocr_response([None], min_text_len=2, empty_message="No text")
        self.assertEqual(status, 422)
        self.assertEqual(payload["message"], "No text")

    def test_success_returns_200_with_text(self):
        sample = [
            [
                [[[0, 0], [1, 0], [1, 1], [0, 1]], ("Ingredients: Water", 0.98)],
            ]
        ]
        status, payload = build_ocr_response(sample, min_text_len=2, empty_message="No text")
        self.assertEqual(status, 200)
        self.assertEqual(payload["text"], "Ingredients: Water")
        self.assertEqual(len(payload["lines"]), 1)


if __name__ == "__main__":
    unittest.main()
