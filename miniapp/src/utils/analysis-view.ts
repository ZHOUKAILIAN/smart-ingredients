export type TableRow = {
  label: string;
  value: string;
};

export type TableSection = {
  title: string;
  rows: TableRow[];
};

export type SummaryViewModel = {
  headline: string;
  warnings: string[];
  scoreText: string;
};

export type IngredientRiskLevel = 'high' | 'medium' | 'low' | 'unknown';

export type IngredientDetailItem = {
  name: string;
  category: string;
  functionText: string;
  riskLevel: IngredientRiskLevel;
  note: string;
};

const INGREDIENT_KEYWORDS = [
  '配料',
  '配料表',
  '成分',
  'ingredient',
  'ingredients',
  'table',
  'ocr text',
  'confirmed text',
  '原料',
  '配方'
];

const SCORE_KEYWORDS = ['评分', 'score', '风险等级', 'risk'];
const SUMMARY_PRIMARY_LABEL_KEYWORDS = [
  '分析摘要',
  '摘要',
  '核心结论',
  '结论',
  'summary',
  'recommendation',
  'overall assessment',
  'overallassessment',
  '总体评估',
  '整体评估'
];
const SUMMARY_HEADLINE_LABEL_KEYWORDS = ['核心结论', '结论', '摘要', '建议', 'recommendation', 'summary', 'overall assessment'];
const SUMMARY_ONLY_LABEL_KEYWORDS = [
  '分析摘要',
  'summary',
  'overall assessment',
  'overallassessment',
  '核心结论',
  '结论',
  '总体评估',
  '整体评估'
];
const SUMMARY_WARNING_LABEL_KEYWORDS = ['风险', '提示', '注意', 'warning', '判断依据', '潜在风险来源', '慎用', '不建议'];
const RISK_HIGH_KEYWORDS = ['高风险', 'high', '慎用', '避免', '不建议', '不宜'];
const RISK_MEDIUM_KEYWORDS = ['中风险', 'medium', '注意'];
const RISK_LOW_KEYWORDS = ['低风险', 'low', '健康'];
const TECHNICAL_NOTE_KEYWORDS = [
  '模型',
  '规则',
  '算法',
  'llm',
  'prompt',
  'token',
  '置信度',
  '推理',
  '判断依据',
  '潜在风险来源',
  'system',
  'chain'
];
const DIET_NOTE_KEYWORDS = [
  '饮食',
  '摄入',
  '食用',
  '饮用',
  '控制',
  '减少',
  '少吃',
  '避免',
  '不宜',
  '频次',
  '糖',
  '盐',
  '钠',
  '脂肪',
  '热量',
  '添加剂',
  '过敏',
  '儿童',
  '孕期',
  '哺乳'
];

const LABEL_ALIAS: Record<string, string> = {
  level: '风险等级',
  risklevel: '风险等级',
  score: '综合评分',
  totalscore: '综合评分',
  reasons: '判断依据',
  reason: '判断依据',
  factorys: '潜在风险来源',
  factors: '潜在风险来源',
  factories: '潜在风险来源',
  advice: '建议',
  suggestion: '建议',
  suggestions: '建议',
  summary: '分析摘要',
  overallassessment: '分析摘要'
};

const normalizeKey = (key: string) => key.replace(/[\s_-]+/g, '').toLowerCase();
const CATEGORY_ALIAS: Record<string, string> = {
  other: '其他',
  others: '其他',
  additive: '添加剂',
  additives: '添加剂',
  flavor: '香精/香料',
  flavour: '香精/香料',
  sweetener: '甜味剂',
  preservative: '防腐剂',
  colorant: '着色剂',
  colouring: '着色剂',
  antioxidant: '抗氧化剂',
  acidulant: '酸度调节剂',
  emulsifier: '乳化剂',
  stabilizer: '稳定剂'
};

const toTitle = (key: string) => {
  const alias = LABEL_ALIAS[normalizeKey(key)];
  if (alias) {
    return alias;
  }

  return key
    .replace(/_/g, ' ')
    .replace(/\s+/g, ' ')
    .trim()
    .replace(/\b\w/g, (ch) => ch.toUpperCase());
};

const normalizeLevelText = (value: string): string => {
  const text = value.trim().toLowerCase();
  if (!text) {
    return '-';
  }
  if (['low', 'lowrisk', 'low_risk'].includes(text)) {
    return '低风险';
  }
  if (['medium', 'mid', 'middle', 'moderate', 'mediumrisk', 'medium_risk'].includes(text)) {
    return '中风险';
  }
  if (['high', 'highrisk', 'high_risk'].includes(text)) {
    return '高风险';
  }
  return value;
};

const withBulletList = (lines: string[]): string => lines.map((line) => `• ${line}`).join('\n');

const toText = (value: any, keyHint?: string): string => {
  const key = keyHint ? normalizeKey(keyHint) : '';
  const isReasonLike = key === 'reasons' || key === 'reason' || key === 'factors' || key === 'factorys' || key === 'factories';
  const isLevelLike = key === 'level' || key === 'risklevel';

  if (value == null || value === '') {
    return '-';
  }
  if (typeof value === 'string' || typeof value === 'number' || typeof value === 'boolean') {
    const raw = String(value);
    return isLevelLike ? normalizeLevelText(raw) : raw;
  }
  if (Array.isArray(value)) {
    if (!value.length) {
      return '-';
    }
    const lines = value.map((item) => toText(item)).filter((item) => item && item !== '-');
    if (!lines.length) {
      return '-';
    }
    return isReasonLike ? withBulletList(lines) : lines.join('\n');
  }
  if (typeof value === 'object') {
    const entries = Object.entries(value);
    if (!entries.length) {
      return '-';
    }
    return entries.map(([k, v]) => `${toTitle(k)}: ${toText(v, k)}`).join('\n');
  }
  return String(value);
};

const containsIngredientKeyword = (value: string): boolean => {
  const text = value.toLowerCase();
  return INGREDIENT_KEYWORDS.some((keyword) => text.includes(keyword));
};

const looksLikeIngredientStructuredValue = (value: string): boolean => {
  const text = value.toLowerCase();
  const nameLike = text.includes('name:') || text.includes('名称:') || text.includes('配料:') || text.includes('成分:');
  const riskLike = text.includes('risk level') || text.includes('风险等级') || text.includes('风险:');
  const funcLike = text.includes('function:') || text.includes('作用:') || text.includes('说明:') || text.includes('描述:');
  return (nameLike && riskLike) || (nameLike && funcLike) || (riskLike && funcLike);
};

const containsKeyword = (value: string, keywords: string[]): boolean => {
  const text = value.toLowerCase();
  return keywords.some((keyword) => text.includes(keyword.toLowerCase()));
};

const isValidText = (value: string): boolean => {
  const text = value.trim();
  return !!text && text !== '-' && !text.includes('暂无');
};

const splitTextLines = (value: string): string[] => {
  if (!isValidText(value)) {
    return [];
  }
  return value
    .split(/\n+/)
    .flatMap((line) => line.split(/[；;]+/))
    .map((line) => line.replace(/^[•·\-\s]+/, '').trim())
    .filter((line) => isValidText(line));
};

const dedupeLines = (lines: string[]): string[] => Array.from(new Set(lines.map((line) => line.trim()).filter(Boolean)));
const isSummaryOnlyLabel = (label: string): boolean => containsKeyword(label, SUMMARY_ONLY_LABEL_KEYWORDS);
const isSummaryOnlyRow = (label: string, value: string): boolean =>
  isSummaryOnlyLabel(label) || containsKeyword(`${label} ${value}`, SUMMARY_ONLY_LABEL_KEYWORDS);

const toDisplayCategory = (value: string): string => {
  const raw = value.trim();
  if (!raw) {
    return '';
  }
  const alias = CATEGORY_ALIAS[normalizeKey(raw)];
  return alias || raw;
};

const normalizeIngredientName = (value: string): string =>
  value
    .toLowerCase()
    .replace(/[\s·•,，。;；:：()（）[\]{}<>《》“”"'/\\|_-]+/g, '')
    .trim();

const isGenericIngredientName = (value: string): boolean => {
  const key = normalizeKey(value);
  if (!key) {
    return true;
  }
  return [
    '内容',
    '条目',
    '配料',
    '配料表',
    '成分',
    'summary',
    'analysis',
    'overallassessment',
    'recommendation',
    'other',
    'additive',
    'ingredient',
    'ingredients',
    'category',
    '分类',
    '类型',
    '类别'
  ].includes(key);
};

const mergeText = (leftRaw: string, rightRaw: string): string => {
  const left = leftRaw.trim();
  const right = rightRaw.trim();
  if (!right) {
    return left;
  }
  if (!left) {
    return right;
  }

  const merged = dedupeLines([...splitTextLines(left), ...splitTextLines(right)]);
  if (merged.length) {
    return merged.join('；');
  }
  if (left.includes(right)) {
    return left;
  }
  if (right.includes(left)) {
    return right;
  }
  return `${left}；${right}`;
};

const isTechnicalNote = (value: string): boolean => {
  const text = value.toLowerCase();
  return TECHNICAL_NOTE_KEYWORDS.some((keyword) => text.includes(keyword.toLowerCase()));
};

const isDietNote = (value: string): boolean => {
  if (isTechnicalNote(value)) {
    return false;
  }
  const text = value.toLowerCase();
  return DIET_NOTE_KEYWORDS.some((keyword) => text.includes(keyword.toLowerCase()));
};

const buildDietFallbackNotes = (rows: TableRow[]): string[] => {
  const context = rows.map((row) => `${row.label} ${row.value}`.toLowerCase()).join(' ');
  const notes: string[] = [];

  if (containsKeyword(context, ['糖', '甜', '果葡', '葡萄糖', '蔗糖'])) {
    notes.push('控制含糖饮料和甜食摄入，避免短时间大量食用。');
  }
  if (containsKeyword(context, ['钠', '盐', '酱油', '味精'])) {
    notes.push('注意每日钠盐总摄入，优先选择清淡烹调。');
  }
  if (containsKeyword(context, ['脂肪', '油', '奶精', '植脂末', '反式'])) {
    notes.push('减少高油高脂食物搭配，避免叠加热量负担。');
  }
  if (containsKeyword(context, ['香精', '色素', '添加剂', '防腐'])) {
    notes.push('减少深加工食品频次，优先选择原型食物。');
  }

  if (!notes.length) {
    notes.push('保持饮食清淡，注意控制总摄入量。');
    notes.push('建议搭配蔬果和充足饮水，避免长期高频食用同类加工食品。');
  }

  return notes;
};

const parseRiskLevel = (value: string): IngredientRiskLevel => {
  const text = value.trim().toLowerCase();
  if (!text) {
    return 'unknown';
  }
  if (RISK_HIGH_KEYWORDS.some((keyword) => text.includes(keyword))) {
    return 'high';
  }
  if (RISK_MEDIUM_KEYWORDS.some((keyword) => text.includes(keyword))) {
    return 'medium';
  }
  if (RISK_LOW_KEYWORDS.some((keyword) => text.includes(keyword))) {
    return 'low';
  }
  return 'unknown';
};

const riskLevelOrder = (level: IngredientRiskLevel): number => {
  switch (level) {
    case 'high':
      return 0;
    case 'medium':
      return 1;
    case 'low':
      return 2;
    default:
      return 3;
  }
};

export const formatRiskLevel = (level: IngredientRiskLevel): string => {
  switch (level) {
    case 'high':
      return '高风险';
    case 'medium':
      return '中风险';
    case 'low':
      return '低风险';
    default:
      return '未知';
  }
};

export const buildSections = (source: any): TableSection[] => {
  if (typeof source === 'string') {
    return [{ title: '分析摘要', rows: [{ label: '内容', value: source }] }];
  }

  if (Array.isArray(source)) {
    return [
      {
        title: '分析结果',
        rows: source.map((item, idx) => ({ label: `条目 ${idx + 1}`, value: toText(item) }))
      }
    ];
  }

  if (source && typeof source === 'object') {
    const rootRows: TableRow[] = [];
    const nestedSections: TableSection[] = [];

    Object.entries(source).forEach(([key, value]) => {
      if (value && typeof value === 'object' && !Array.isArray(value)) {
        const childRows = Object.entries(value as Record<string, any>).map(([childKey, childValue]) => ({
          label: toTitle(childKey),
          value: toText(childValue, childKey)
        }));
        if (childRows.length) {
          nestedSections.push({ title: toTitle(key), rows: childRows });
          return;
        }
      }

      rootRows.push({ label: toTitle(key), value: toText(value, key) });
    });

    if (rootRows.length) {
      nestedSections.unshift({ title: '分析摘要', rows: rootRows });
    }

    return nestedSections.length ? nestedSections : [{ title: '分析结果', rows: [{ label: '内容', value: '-' }] }];
  }

  return [{ title: '分析结果', rows: [{ label: '内容', value: toText(source) }] }];
};

export const splitSectionsByTab = (
  sections: TableSection[]
): { summary: TableSection[]; ingredients: TableSection[] } => {
  const summary: TableSection[] = [];
  const ingredients: TableSection[] = [];

  sections.forEach((section) => {
    const sectionHasIngredient =
      containsIngredientKeyword(section.title) ||
      section.rows.some(
        (row) =>
          containsIngredientKeyword(row.label) ||
          looksLikeIngredientStructuredValue(row.value)
      );

    if (sectionHasIngredient) {
      ingredients.push(section);
    } else {
      summary.push(section);
    }
  });

  if (!summary.length) {
    summary.push({
      title: '分析摘要',
      rows: [{ label: '内容', value: '暂无摘要信息' }]
    });
  }

  if (!ingredients.length) {
    ingredients.push({
      title: '配料表',
      rows: [{ label: '内容', value: '暂无配料表信息' }]
    });
  }

  return { summary, ingredients };
};

export const pickScoreText = (summarySections: TableSection[]): string => {
  for (const section of summarySections) {
    for (const row of section.rows) {
      const key = `${section.title} ${row.label}`.toLowerCase();
      if (SCORE_KEYWORDS.some((kw) => key.includes(kw.toLowerCase()))) {
        return row.value || '-';
      }
    }
  }
  return '-';
};

export const pickSummaryPreview = (summarySections: TableSection[], maxItems = 4): TableRow[] => {
  const rows = summarySections.flatMap((section) => section.rows);
  return rows.slice(0, Math.max(1, maxItems));
};

export const buildSummaryView = (summarySections: TableSection[]): SummaryViewModel => {
  const rows = summarySections.flatMap((section) => section.rows);
  const scoreText = pickScoreText(summarySections);

  const headlineRow =
    rows.find((row) => containsKeyword(row.label, SUMMARY_PRIMARY_LABEL_KEYWORDS) && isValidText(row.value) && !isTechnicalNote(row.value)) ||
    rows.find((row) => containsKeyword(row.label, SUMMARY_HEADLINE_LABEL_KEYWORDS) && isValidText(row.value)) ||
    rows.find(
      (row) =>
        isValidText(row.value) &&
        !containsIngredientKeyword(row.label) &&
        !looksLikeIngredientStructuredValue(row.value) &&
        !containsKeyword(row.label, SCORE_KEYWORDS) &&
        !containsKeyword(row.label, SUMMARY_WARNING_LABEL_KEYWORDS) &&
        !isTechnicalNote(`${row.label} ${row.value}`)
    ) ||
    rows.find((row) => isValidText(row.value) && !isTechnicalNote(`${row.label} ${row.value}`));

  const headline = headlineRow?.value?.trim() || '已完成配料分析，请查看详细配料表。';

  const warningRows = rows.filter(
    (row) => containsKeyword(row.label, SUMMARY_WARNING_LABEL_KEYWORDS) && isValidText(row.value)
  );

  let warnings = dedupeLines(warningRows.flatMap((row) => splitTextLines(row.value)));

  if (!warnings.length) {
    warnings = dedupeLines(
      rows
        .filter(
          (row) =>
            row !== headlineRow &&
            isValidText(row.value) &&
            containsKeyword(`${row.label} ${row.value}`, ['风险', '注意', '建议', '慎用'])
        )
        .flatMap((row) => splitTextLines(row.value))
    );
  }

  warnings = warnings.filter((line) => line !== headline && !isTechnicalNote(line));
  const dietWarnings = warnings.filter((line) => isDietNote(line));
  warnings = (dietWarnings.length ? dietWarnings : buildDietFallbackNotes(rows)).slice(0, 4);

  return {
    headline,
    warnings,
    scoreText
  };
};

export const buildIngredientDetailItems = (ingredientSections: TableSection[]): IngredientDetailItem[] => {
  const normalizeFieldKey = (key: string): string => key.replace(/[\s_]+/g, '').toLowerCase();
  const isNameKey = (key: string): boolean =>
    ['name', 'ingredient', 'ingredients', 'ingredientname', '名称', '配料', '成分', '原料'].includes(key);
  const isCategoryKey = (key: string): boolean => ['category', 'type', 'group', '类型', '类别', '分类'].includes(key);
  const isFunctionKey = (key: string): boolean =>
    ['function', 'description', 'desc', 'effect', '作用', '说明', '描述', '分析'].includes(key);
  const isRiskKey = (key: string): boolean => ['risk', 'risklevel', '风险', '风险等级'].includes(key);
  const isNoteKey = (key: string): boolean => ['note', 'remark', 'remarks', '备注', '建议'].includes(key);

  const parseStructuredFields = (value: string): Partial<IngredientDetailItem> => {
    const lines = value.split(/\n+/).map((line) => line.trim()).filter(Boolean);
    const parsed: Partial<IngredientDetailItem> = {};

    lines.forEach((line) => {
      const match = line.match(/^([^:：]+)[:：]\s*(.+)$/);
      if (!match) {
        return;
      }
      const rawKey = normalizeFieldKey(match[1]);
      const fieldValue = match[2].trim();

      if (!fieldValue) {
        return;
      }

      if (isNameKey(rawKey)) {
        parsed.name = fieldValue;
        return;
      }
      if (isCategoryKey(rawKey)) {
        parsed.category = fieldValue;
        return;
      }
      if (isFunctionKey(rawKey)) {
        parsed.functionText = fieldValue;
        return;
      }
      if (isRiskKey(rawKey)) {
        parsed.riskLevel = parseRiskLevel(fieldValue);
        return;
      }
      if (isNoteKey(rawKey)) {
        parsed.note = fieldValue;
      }
    });

    return parsed;
  };

  const parseStructuredMultiItems = (value: string): Partial<IngredientDetailItem>[] => {
    const lines = value.split(/\n+/).map((line) => line.trim()).filter(Boolean);
    const chunks: Partial<IngredientDetailItem>[] = [];
    let current: Partial<IngredientDetailItem> | null = null;

    const pushCurrent = () => {
      if (!current) {
        return;
      }
      if (current.name || current.functionText || current.note || current.category || current.riskLevel) {
        chunks.push(current);
      }
      current = null;
    };

    lines.forEach((line) => {
      const match = line.match(/^([^:：]+)[:：]\s*(.+)$/);
      if (!match) {
        return;
      }

      const rawKey = normalizeFieldKey(match[1]);
      const fieldValue = match[2].trim();
      if (!fieldValue) {
        return;
      }

      if (isNameKey(rawKey)) {
        pushCurrent();
        current = { name: fieldValue };
        return;
      }

      if (!current) {
        current = {};
      }

      if (isCategoryKey(rawKey)) {
        current.category = fieldValue;
        return;
      }
      if (isFunctionKey(rawKey)) {
        current.functionText = fieldValue;
        return;
      }
      if (isRiskKey(rawKey)) {
        current.riskLevel = parseRiskLevel(fieldValue);
        return;
      }
      if (isNoteKey(rawKey)) {
        current.note = fieldValue;
      }
    });

    pushCurrent();
    return chunks;
  };

  const items = ingredientSections.flatMap((section) =>
    section.rows
      .filter(
        (row) =>
          isValidText(row.value) &&
          !isSummaryOnlyRow(row.label, row.value) &&
          (containsIngredientKeyword(`${section.title} ${row.label}`) || looksLikeIngredientStructuredValue(row.value))
      )
      .flatMap((row) => {
        const sectionCategory = toDisplayCategory(section.title === '配料表' ? '' : section.title);

        const multiParsed = parseStructuredMultiItems(row.value);
        if (multiParsed.length) {
          return multiParsed
            .map((parsed) => {
              const name = (parsed.name?.trim() || row.label || '').trim();
              if (isGenericIngredientName(name)) {
                return null;
              }
              return {
                name,
                category: toDisplayCategory(parsed.category || sectionCategory),
                functionText: (parsed.functionText || '暂无说明').trim(),
                riskLevel: parsed.riskLevel || parseRiskLevel(row.value),
                note: (parsed.note || '').trim()
              } as IngredientDetailItem;
            })
            .filter((item): item is IngredientDetailItem => !!item);
        }

        const lines = splitTextLines(row.value);
        const structured = parseStructuredFields(row.value);
        const name = (structured.name?.trim() || row.label || '').trim();
        if (isGenericIngredientName(name)) {
          return [];
        }

        const functionText = (structured.functionText || lines[0] || row.value.trim()).trim();
        const note = (structured.note || lines.slice(1).join('；')).trim();
        const riskLevel = structured.riskLevel || parseRiskLevel(`${row.label} ${row.value}`);
        const category = toDisplayCategory(structured.category || sectionCategory);

        return {
          name,
          category,
          functionText: functionText || '暂无说明',
          riskLevel,
          note
        };
      })
  );

  type ItemBucket = IngredientDetailItem & { order: number };
  const mergedMap = new Map<string, ItemBucket>();

  items.forEach((item, index) => {
    const normalizedKey = normalizeIngredientName(item.name);
    const key = normalizedKey || `__row_${index}`;
    const normalizedItem: IngredientDetailItem = {
      name: item.name.trim(),
      category: toDisplayCategory(item.category),
      functionText: item.functionText.trim() || '暂无说明',
      riskLevel: item.riskLevel,
      note: item.note.trim()
    };

    const existed = mergedMap.get(key);
    if (!existed) {
      mergedMap.set(key, { ...normalizedItem, order: index });
      return;
    }

    if (!existed.category && normalizedItem.category) {
      existed.category = normalizedItem.category;
    }
    if (riskLevelOrder(normalizedItem.riskLevel) < riskLevelOrder(existed.riskLevel)) {
      existed.riskLevel = normalizedItem.riskLevel;
    }
    existed.functionText = mergeText(existed.functionText, normalizedItem.functionText) || '暂无说明';
    existed.note = mergeText(existed.note, normalizedItem.note);

    if (normalizedItem.name.length > existed.name.length) {
      existed.name = normalizedItem.name;
    }
  });

  return Array.from(mergedMap.values())
    .sort((a, b) => {
      const byRisk = riskLevelOrder(a.riskLevel) - riskLevelOrder(b.riskLevel);
      if (byRisk !== 0) {
        return byRisk;
      }
      return a.order - b.order;
    })
    .map(({ order, ...item }) => item);
};
