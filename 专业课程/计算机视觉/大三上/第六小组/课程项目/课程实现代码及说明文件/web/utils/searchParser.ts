
export interface SearchFilter {
    type: 'date' | 'time';
    label: string;
    originalText: string;
}

export interface ParsedSearch {
    cleanQuery: string;
    filters: SearchFilter[];
}

const DATE_PATTERNS: Array<{ regex: RegExp; label: string }> = [
    { regex: /(最近一周|最近7天|last week)/i, label: '最近7天' },
    { regex: /(最近30天|最近一月|最近1月|last month)/i, label: '最近30天' },
    { regex: /(上周|上星期)/i, label: '上周' },
    { regex: /(本周|这周|this week)/i, label: '本周' },
    { regex: /(本月|this month)/i, label: '本月' },
    { regex: /(上个月)/i, label: '上个月' },
    { regex: /(昨天|yesterday)/i, label: '昨天' },
    { regex: /(今天|今日|today)/i, label: '今天' },
    { regex: /(半年前|半年内|最近半年)/i, label: '半年内' },
    { regex: /(去年|last year)/i, label: '去年' },
    { regex: /(今年|this year)/i, label: '今年' },
];

const TIME_PATTERNS: Array<{ regex: RegExp; label: string }> = [
    { regex: /(上午|morning)/i, label: '上午 (06-12)' },
    { regex: /(中午|noon)/i, label: '中午 (11-13)' },
    { regex: /(下午|afternoon)/i, label: '下午 (12-18)' },
    { regex: /(傍晚|evening)/i, label: '傍晚 (17-20)' },
    { regex: /(晚上|night)/i, label: '晚上 (18-22)' },
    { regex: /(夜间|夜里)/i, label: '夜间 (22-06)' },
    { regex: /(凌晨)/i, label: '凌晨 (00-06)' },
];

const CLEANUP_PATTERNS = [
    /(拍摄的|拍摄)/g
];

export const parseSearchQuery = (query: string): ParsedSearch => {
    let cleanQuery = query;
    const filters: SearchFilter[] = [];

    // 1. Extract Date Patterns
    DATE_PATTERNS.forEach(pattern => {
        const match = cleanQuery.match(pattern.regex);
        if (match) {
            filters.push({
                type: 'date',
                label: pattern.label,
                originalText: match[0]
            });
            cleanQuery = cleanQuery.replace(pattern.regex, ' ');
        }
    });

    // 2. Extract Time Patterns
    TIME_PATTERNS.forEach(pattern => {
        const match = cleanQuery.match(pattern.regex);
        if (match) {
            filters.push({
                type: 'time',
                label: pattern.label,
                originalText: match[0]
            });
            cleanQuery = cleanQuery.replace(pattern.regex, ' ');
        }
    });

    // 3. Cleanup unused words
    CLEANUP_PATTERNS.forEach(regex => {
        cleanQuery = cleanQuery.replace(regex, ' ');
    });

    // 4. Final trim
    cleanQuery = cleanQuery.replace(/\s+/g, ' ').trim();

    return { cleanQuery, filters };
};
