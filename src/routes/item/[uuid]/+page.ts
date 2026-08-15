import { fetch_item_with_uuid } from "$lib/core/commands";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params, depends }) => {
  const uuid = params.uuid.trim();

  const result = await fetch_item_with_uuid(uuid);
  if (!result.ok) error(500, result.data);
  if (!result.data)
    error(404, {
      message: `${uuid} not found in database`,
    });

  depends(`item:${uuid}`);

  return {
    item: result.data,
  };
};
