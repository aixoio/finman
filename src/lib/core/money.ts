const CENTS_PER_DOLLAR = 100;

export const MAX_MONEY_CENTS = Number.MAX_SAFE_INTEGER;

interface MoneyValidationOptions {
  allow_zero?: boolean;
  allow_rounding?: boolean;
}

export function is_valid_dollar_amount(
  amount: unknown,
  {
    allow_zero = true,
    allow_rounding = false,
  }: MoneyValidationOptions = {},
): amount is number {
  if (typeof amount !== "number" || !Number.isFinite(amount) || amount < 0) {
    return false;
  }

  if (!allow_zero && amount === 0) return false;

  const cents = amount * CENTS_PER_DOLLAR;
  const rounded_cents = Math.round(cents);

  return (
    Number.isSafeInteger(rounded_cents) &&
    (allow_rounding || Math.abs(cents - rounded_cents) < 1e-7)
  );
}

export function dollars_to_cents(
  amount: number,
  options?: MoneyValidationOptions,
): number {
  if (!is_valid_dollar_amount(amount, options)) {
    throw new RangeError("amount is outside the supported range or precision");
  }

  return Math.round(amount * CENTS_PER_DOLLAR);
}

export function cents_to_dollars(cents: number): number {
  if (!Number.isSafeInteger(cents)) {
    throw new RangeError("cents must be a safe integer");
  }

  return cents / CENTS_PER_DOLLAR;
}

export function format_dollars(amount: number): string {
  return Number.isFinite(amount) ? amount.toFixed(2) : "—";
}

export function format_cents(cents: number): string {
  return format_dollars(cents_to_dollars(cents));
}

export function normalize_dollars(
  amount: number,
  options?: MoneyValidationOptions,
): number {
  return cents_to_dollars(
    dollars_to_cents(amount, { ...options, allow_rounding: true }),
  );
}
