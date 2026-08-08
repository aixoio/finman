export enum ItemType {
  Savings = "Savings",
  SelfLoan = "SelfLoan",
  ExternalLoan = "ExternalLoan",
}

export interface Item {
  uuid: string;
  name: string;
  comment: string | null;
  item_type: ItemType;
  target_cents: number;
  current_cents: number;
  archived: boolean;
  created_at: string;
  updated_at: string;
}

export enum AppErrorType {
  DatabaseError = "DatabaseError",
}

export interface AppError {
  type: AppErrorType;
  message: string;
}

export type AppOk<V> = { kind: "ok"; data: V };
export type AppErr = { kind: "err"; data: AppError };

export type AppResult<V> = AppOk<V> | AppErr;
