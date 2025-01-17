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
    stack = []
    for page in reader.pages:
        for line in page.extract_text().splitlines():
            line = line.strip()
            if line.startswith("Câu "):
                #câu 457 749 943 không có câu hỏi
                question = line.split(": ", maxsplit=1)
                stack.append(question[1] if len(question) > 1 else "")
            else:
                if line.startswith("A. "):
                    stack.append(line[3:])
                elif line.startswith("B. "):
                    stack.append(line[3:])
                elif line.startswith("C. "):
                    stack.append(line[3:])
                elif line.startswith("D. "):
                    stack.append(line[3:])
                elif line.startswith("Đáp án: "):          
                    question = {
                        "question": stack[0],
                        "choices": stack[1:],
                        "answer": ord(line[8]) - ord("A")
                    }
                    questions.append(question)
                    stack = []
                else:
                    if stack:
                        stack[-1] += " " + line.strip()

    with open(cwd_path / "questions.json", "w", encoding="utf-8") as f:
        json.dump(questions, f, indent=4, ensure_ascii=False)


if __name__ == "__main__":
    main()
