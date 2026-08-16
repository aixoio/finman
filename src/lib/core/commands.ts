import { invalidate } from "$app/navigation";
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

    await invalidate("items:not_archived");

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

export type UpdateItemActionCompleteGoal = {
  type: "CompleteGoal";
};
export type UpdateItemActionSetExact = {
  type: "SetExact";
  data: {
    amount_cents: number;
  };
};
export type UpdateItemActionAdd = {
  type: "Add";
  data: {
    amount_cents: number;
  };
};
export type UpdateItemActionSubtract = {
  type: "Subtract";
  data: {
    amount_cents: number;
  };
};
export type UpdateItemActionEdit = {
  type: "Edit";
  data: {
    name: string;
    target_cents: number;
    current_cents: number;
  };
};
export type UpdateItemActionComment = {
  type: "Comment";
  data: {
    comment: Option<string>;
  };
};
export type UpdateItemActionArchive = {
  type: "Archive";
  data: {
    archived: boolean;
  };
};

export type ItemUpdateAction =
  | UpdateItemActionCompleteGoal
  | UpdateItemActionSetExact
  | UpdateItemActionAdd
  | UpdateItemActionSubtract
  | UpdateItemActionEdit
  | UpdateItemActionComment
  | UpdateItemActionArchive;

export type UnitType = {};

export async function update_item_with_uuid(
  uuid: string,
  action: ItemUpdateAction,
): Promise<AppResult<UnitType>> {
  try {
    await invoke("update_item_with_uuid", {
      uuid,
      action,
    });

    await Promise.all([
      invalidate(`item:${uuid}`),
      invalidate("items:not_archived"),
      invalidate("items:archived"),
    ]);

    return {
      ok: true,
      data: {},
    };
  } catch (error: unknown) {
    if (!is_app_error(error)) throw error;

    return {
      ok: false,
      data: error,
    };
  }
}

export async function delete_item_with_uuid(
  uuid: string,
): Promise<AppResult<UnitType>> {
  try {
    await invoke("delete_item_with_uuid", {
      uuid,
    });

    await Promise.all([
      invalidate("items:not_archived"),
      invalidate("items:archived"),
    ]);

    return {
      ok: true,
      data: {},
    };
  } catch (error: unknown) {
    if (!is_app_error(error)) throw error;

    return {
      ok: false,
      data: error,
    };
  }
}
