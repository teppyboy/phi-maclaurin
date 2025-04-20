import json
import sys


def main():
    file_1 = sys.argv[1]
    file_2 = sys.argv[2]
    with open(file_1, "r", encoding="utf-8") as f:
        legacy_data = json.load(f)
    new_data = []
    for item in legacy_data:
        new_item = {
            "question": item["question"],
            "choices": item["choices"],
            "answer": [item["answer"]]
        }
        new_data.append(new_item)
    with open(file_2, "w", encoding="utf-8") as f:
        json.dump(new_data, f, indent=4, ensure_ascii=False)
    print(f"Converted {len(legacy_data)} items from {file_1} to {file_2}")


if __name__ == "__main__":
    main()