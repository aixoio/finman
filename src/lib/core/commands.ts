import { invoke } from "@tauri-apps/api/core";

export enum ItemType {
  Savings = "Savings",
  SelfLoan = "SelfLoan",
  ExternalLoan = "ExternalLoan",
}

export function display_format_item_type(item_type: ItemType): string {
  switch (item_type) {
    case ItemType.Savings:
      return "Savings";
    case ItemType.SelfLoan:
      return "Self Loan";
    case ItemType.ExternalLoan:
      return "External Loan";
  }
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
  InputError = "InputError",
  SerdeError = "SerdeError",
}

export interface AppError {
  type: AppErrorType;
  message: string;
}

function is_app_error(error: unknown): error is AppError {
  if (!error || typeof error !== "object") return false;

  const { type, message } = error as Record<string, unknown>;

  return (
    typeof message === "string" &&
    Object.values(AppErrorType).includes(type as AppErrorType)
  );
}

export type AppOk<V> = { ok: true; data: V };
export type AppErr = { ok: false; data: AppError };

export type AppResult<V> = AppOk<V> | AppErr;

export type Option<T> = T | null;

export async function select_all_items_not_archived(): Promise<
  AppResult<Item[]>
> {
  try {
    const data: Item[] = await invoke("select_all_items_not_archived");

    return {
      ok: true,
      data,
    };
  } catch (err: unknown) {
    if (!is_app_error(err)) throw err;

    return {
      ok: false,
      data: err,
    };
  }
}

export async function insert_item(
  name: string,
  comment: string | null,
  item_type: ItemType,
  target_cents: number,
  current_cents: number,
): Promise<AppResult<string>> {
  try {
    const uuid: string = await invoke("insert_item", {
      name,
      comment,
      itemType: item_type,
      targetCents: target_cents,
      currentCents: current_cents,
    });

    return {
      ok: true,
      data: uuid,
    };
  } catch (error: unknown) {
    if (!is_app_error(error)) throw error;

    return {
      ok: false,
      data: error,
    };
  }
}

export async function fetch_item_with_uuid(
  uuid: string,
): Promise<AppResult<Option<Item>>> {
  try {
    const option: Option<Item> = await invoke("fetch_item_with_uuid", {
      uuid,
    });

    return {
      ok: true,
      data: option,
    };
  } catch (error: unknown) {
    if (!is_app_error(error)) throw error;

    return {
      ok: false,
      data: error,
    };
  }
}
