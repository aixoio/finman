import { select_all_items_not_archived } from "$lib/core/commands";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  const result = await select_all_items_not_archived();
  if (!result.ok) {
    error(500, JSON.stringify(result.data));
  }

  return {
    items: result.data,
  };
};
