import { PDFDocument, StandardFonts } from "pdf-lib";
import { readdir, readFile, mkdir, writeFile } from "fs/promises";
import { basename, extname, join } from "path";

const QUESTION_COUNT = 30;

type Entry = {
  sourceId: number;
  question: string;
  answers: string[];
};

type Counts = Record<string, number>;

const scriptDir = decodeURIComponent(new URL(".", import.meta.url).pathname);
const PROJECT_ROOT = join(scriptDir, "..");

async function main() {
  const cardsDir = join(PROJECT_ROOT, "cards");
  const paths = await findCardFiles(cardsDir);
  const entries: Entry[] = [];

  for (const path of paths) {
    entries.push(...(await parseTextFile(path)));
  }

  const countsPath = join(PROJECT_ROOT, "cards", "card-data.json");
  const counts = await loadCardCounts(countsPath);
  const questions = chooseN(entries, QUESTION_COUNT, counts);
  const outputPath = join(PROJECT_ROOT, "output", "questions.pdf");

  await generatePdf(questions, outputPath);
  await updateCardCounts(countsPath, questions);

  console.log(`Parsed ${entries.length} entries from ${paths.length} file(s)`);
  console.log(`Wrote PDF to ${outputPath}`);
  console.log(`Updated card counts in ${countsPath}`);
}

async function generatePdf(questions: Entry[], path: string) {
  await mkdir(join(path, ".."), { recursive: true });

  const pageWidth = 595;
  const pageHeight = 842;
  const margin = 40;
  const fontSize = 11;
  const lineHeight = 14;
  const maxChars = Math.floor((pageWidth - margin * 2) / 6.5);
  const markReserve = 10;
  const dotLine = ".".repeat(maxChars);

  const doc = await PDFDocument.create();
  const courier = await doc.embedFont(StandardFonts.Courier);

  const questionPages: Array<Array<{ text: string; mark?: string }>> = [[]];
  let currentY = pageHeight - margin;

  for (const [index, entry] of questions.entries()) {
    const questionNumber = index + 1;
    const questionText = `${questionNumber}. ${entry.question}`;
    const wrappedQuestion = wrapText(questionText, maxChars - markReserve);
    const dotLines = Math.max(entry.answers.length * 2 - 1, 0);
    const requiredSpace = wrappedQuestion.length * lineHeight + dotLines * lineHeight + lineHeight * 0.5;

    if (currentY - requiredSpace < margin) {
      questionPages.push([]);
      currentY = pageHeight - margin;
    }

    for (const [lineIndex, line] of wrappedQuestion.entries()) {
      questionPages[questionPages.length - 1].push({
        text: line,
        mark: lineIndex === 0 ? `[${entry.answers.length}]` : undefined,
      });
      currentY -= lineHeight;
    }

    for (let i = 0; i < dotLines; i++) {
      questionPages[questionPages.length - 1].push({ text: dotLine });
      currentY -= lineHeight;
    }

    currentY -= lineHeight * 0.5;
  }

  for (const pageLines of questionPages) {
    const page = doc.addPage([pageWidth, pageHeight]);

    for (const [lineIndex, { text, mark }] of pageLines.entries()) {
      const y = pageHeight - margin - lineIndex * lineHeight;
      page.drawText(text, { x: margin, y, size: fontSize, font: courier });

      if (mark) {
        const markWidth = courier.widthOfTextAtSize(mark, fontSize);
        page.drawText(mark, { x: pageWidth - margin - markWidth, y, size: fontSize, font: courier });
      }
    }
  }

  const answerPages: Array<Array<{ text: string; mark?: string }>> = [[{ text: "Answers" }, { text: "" }]];
  currentY = pageHeight - margin - lineHeight * 2;

  for (const [index, entry] of questions.entries()) {
    const questionNumber = index + 1;
    const headerText = `${questionNumber}. ${entry.question}`;
    const wrappedHeader = wrapText(headerText, maxChars - markReserve);
    const answerLines: string[] = [];

    for (const answer of entry.answers) {
      const wrapped = wrapText(answer, maxChars - 4);
      for (const wrappedLine of wrapped) {
        answerLines.push(`    ${wrappedLine}`);
      }
    }

    const requiredSpace = wrappedHeader.length * lineHeight + answerLines.length * lineHeight + lineHeight * 0.5;

    if (currentY - requiredSpace < margin) {
      answerPages.push([]);
      currentY = pageHeight - margin;
    }

    for (const [lineIndex, line] of wrappedHeader.entries()) {
      answerPages[answerPages.length - 1].push({
        text: line,
        mark: lineIndex === 0 ? `[${entry.answers.length}]` : undefined,
      });
      currentY -= lineHeight;
    }

    for (const line of answerLines) {
      answerPages[answerPages.length - 1].push({ text: line });
      currentY -= lineHeight;
    }

    currentY -= lineHeight * 0.5;
  }

  for (const pageLines of answerPages) {
    const page = doc.addPage([pageWidth, pageHeight]);

    for (const [lineIndex, { text, mark }] of pageLines.entries()) {
      if (!text) continue;
      const y = pageHeight - margin - lineIndex * lineHeight;
      page.drawText(text, { x: margin, y, size: fontSize, font: courier });

      if (mark) {
        const markWidth = courier.widthOfTextAtSize(mark, fontSize);
        page.drawText(mark, { x: pageWidth - margin - markWidth, y, size: fontSize, font: courier });
      }
    }
  }

  const pdfBytes = await doc.save();
  await writeFile(path, pdfBytes);
}

function wrapText(text: string, maxChars: number): string[] {
  const words = text.split(/\s+/);
  const lines: string[] = [];
  let current = "";

  for (const word of words) {
    if (!current) {
      current = word;
    } else if (current.length + 1 + word.length <= maxChars) {
      current += ` ${word}`;
    } else {
      lines.push(current);
      current = word;
    }
  }

  if (current) {
    lines.push(current);
  }

  return lines.length > 0 ? lines : [""];
}

async function findCardFiles(dir: string): Promise<string[]> {
  const entries = await readdir(dir, { withFileTypes: true });
  const paths = entries
    .filter((entry) => entry.isFile() && extname(entry.name).toLowerCase() === ".txt")
    .map((entry) => join(dir, entry.name))
    .sort();

  return paths;
}

async function parseTextFile(path: string): Promise<Entry[]> {
  const contents = await readFile(path, "utf-8");
  const sourceId = extractSourceId(path);
  const entries: Entry[] = [];
  let currentQuestion: string | null = null;
  let currentAnswers: string[] = [];

  for (const rawLine of contents.split(/\r?\n/)) {
    const line = rawLine.replace(/\r?$/, "");
    if (line.trim() === "") {
      continue;
    }

    const isAnswerLine = line.startsWith(" ") || line.startsWith("\t");
    if (isAnswerLine) {
      if (currentQuestion !== null) {
        currentAnswers.push(line.trimStart());
      }
      continue;
    }

    if (currentQuestion !== null) {
      entries.push({ sourceId, question: currentQuestion, answers: currentAnswers });
      currentAnswers = [];
    }

    const [questionText, answerText] = splitQuestionAnswer(line);
    currentQuestion = questionText;
    if (answerText !== undefined) {
      currentAnswers.push(answerText);
    }
  }

  if (currentQuestion !== null) {
    entries.push({ sourceId, question: currentQuestion, answers: currentAnswers });
  }

  return entries;
}

function splitQuestionAnswer(line: string): [string, string?] {
  const arrowIndex = line.indexOf("→");
  if (arrowIndex >= 0) {
    const left = line.slice(0, arrowIndex).trim();
    const right = line.slice(arrowIndex + 1).trim();
    return [left, right];
  }

  return [line.trim()];
}

function extractSourceId(path: string): number {
  const name = basename(path, extname(path));
  const digits = name.replace(/\D/g, "");
  const parsed = Number(digits);
  return Number.isFinite(parsed) ? parsed : 0;
}

async function loadCardCounts(path: string): Promise<Counts> {
  try {
    const contents = await readFile(path, "utf-8");
    if (contents.trim() === "") {
      return {};
    }
    return JSON.parse(contents) as Counts;
  } catch (error) {
    if ((error as { code?: string }).code === "ENOENT") {
      return {};
    }
    throw error;
  }
}

async function updateCardCounts(path: string, questions: Entry[]) {
  await mkdir(join(path, ".."), { recursive: true });
  const counts = await loadCardCounts(path);

  for (const entry of questions) {
    const key = String(entry.sourceId);
    counts[key] = (counts[key] ?? 0) + 1;
  }

  await writeFile(path, JSON.stringify(counts, null, 2) + "\n");
}

function chooseN(entries: Entry[], n: number, counts: Counts): Entry[] {
  const items = [...entries];
  const sampleN = Math.min(n, items.length);
  const selected: Entry[] = [];

  while (selected.length < sampleN && items.length > 0) {
    const weights = items.map((entry) => {
      const count = counts[String(entry.sourceId)] ?? 0;
      return 1 / (count + 1);
    });

    const total = weights.reduce((sum, value) => sum + value, 0);
    let target = Math.random() * total;
    let index = 0;
    while (target > 0 && index < weights.length) {
      target -= weights[index];
      if (target <= 0) break;
      index += 1;
    }

    if (index >= items.length) {
      index = items.length - 1;
    }

    selected.push(items.splice(index, 1)[0]);
  }

  shuffle(selected);
  return selected;
}

function shuffle<T>(array: T[]) {
  for (let i = array.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [array[i], array[j]] = [array[j], array[i]];
  }
}

main().catch((error) => {
  console.error(error);
  process.exit(1);
});
