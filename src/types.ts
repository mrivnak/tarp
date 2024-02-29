export interface Line {
    content: string;
    coverage: "covered" | "uncovered" | "ignored";
}

export interface SimpleFileReport {
    path: string;
    lines: Line[];
    covered: number;
    coverable: number;
}

export interface SimpleReport {
    files: SimpleFileReport[];
    coverage: number;
}