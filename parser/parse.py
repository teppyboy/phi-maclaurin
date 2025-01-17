import json
import io
from pypdf import PdfReader
from pathlib import Path

def print_to_string(*args, **kwargs):
    output = io.StringIO()
    print(*args, file=output, **kwargs)
    contents = output.getvalue()
    output.close()
    return contents


def main():
    questions: list[dict] = []
    cwd_path = Path(__file__).resolve().parent
    reader = PdfReader(cwd_path / "1100 câu Triết học Mác Lênin.pdf")
    question: dict = None
    for page in reader.pages:
        for line in page.extract_text().splitlines():
            if line.startswith("Câu "):
                question = {
                    "question": line.split(": ", maxsplit=1)[1].strip(),
                    "choices": [
                        
                    ],
                    "answer": 0
                }
            else:
                if line.startswith("A. "):
                    question["choices"].append(line[3:].strip())
                elif line.startswith("B. "):
                    question["choices"].append(line[3:].strip())
                elif line.startswith("C. "):
                    question["choices"].append(line[3:].strip())
                elif line.startswith("D. "):
                    question["choices"].append(line[3:].strip())
                elif line.startswith("Đáp án: "):
                    question["answer"] = ord(line[8]) - ord("A")
                    questions.append(question)
    with open(cwd_path / "questions.json", "w", encoding="utf-8") as f:
        json.dump(questions, f, indent=4, ensure_ascii=False)


if __name__ == "__main__":
    main()
