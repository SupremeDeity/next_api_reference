import { prisma } from "@/lib/prismadb";
import { withApiAuthRequired } from "@auth0/nextjs-auth0";
import { NextRequest, NextResponse } from "next/server";
import { API_ORGANIZATION_PATH } from "@/app/api/api_constants";
import { logger } from "@/lib/logger";
import { itemInfoSchema } from "@/app/dashboard/(main)/inventory/items/new/schema";
import { createId } from "@paralleldrive/cuid2";
import {
  ID_ITEM_PREFIX,
  ID_PURCHASE_INFO_PREFIX,
  ID_SALES_INFO_PREFIX,
} from "@/lib/id_prefix";
import { ItemType, ItemUnit, Prisma } from "@prisma/client";

export function PATCH() {}

export const GET = withApiAuthRequired(async (req: NextRequest) => {
  const newHeaders = new Headers(req.headers);
  newHeaders.set("Authorization", `BEARER ${process.env.INTERNAL_SECRET}`);
  newHeaders.delete("content-length");
  var organization = await fetch(API_ORGANIZATION_PATH, {
    headers: newHeaders,
  });
  const organizationJson = await organization.json();

  try {
    logger.verbose(`GET /api/items ${JSON.stringify(req)}`, {
      organizationId: organizationJson.id,
    });
    const items = await prisma.item.findMany({
      where: { organizationId: organizationJson.id },
    });
    logger.verbose(`Got items ${JSON.stringify(items)}`, {
      organizationId: organizationJson.id,
    });

    return NextResponse.json(items, { status: 200 });
  } catch (e) {
    logger.error(e);

    return NextResponse.json(
      { error: { message: "Server Error" } },
      { status: 500 }
    );
  }
});

const POST = withApiAuthRequired(async (req: NextRequest) => {
  const message = await req.json();

  const newHeaders = new Headers(req.headers);
  newHeaders.set("Authorization", `BEARER ${process.env.INTERNAL_SECRET}`);
  newHeaders.delete("content-length");
  var organization = await fetch(API_ORGANIZATION_PATH, {
    headers: newHeaders,
  });
  const organizationJson = await organization.json();

  const itemInfo = itemInfoSchema.parse(message);

  logger.verbose(`POST /api/item/create ${JSON.stringify(req)}`, {
    itemInfo,
    organization,
  });

  try {
    const createdItem = await prisma.item.create({
      data: {
        id: ID_ITEM_PREFIX + createId(),
        name: itemInfo.name,
        type: itemInfo.type.toUpperCase() as ItemType,
        sku: itemInfo.sku,
        unit: itemInfo.unit.toUpperCase() as ItemUnit,
        returnable: itemInfo.returnable,
        organization: { connect: { id: organizationJson.id } },
        purchaseInfo: itemInfo.purchaseInfo?.isPurchaseInfoSelected
          ? {
              create: {
                id: ID_PURCHASE_INFO_PREFIX + createId(),
                description: itemInfo.purchaseInfo?.description,
                price: new Prisma.Decimal(itemInfo.purchaseInfo?.cost!),
                currency: "PKR",
              },
            }
          : undefined,
        salesInfo: itemInfo.salesInfo?.isSaleInfoSelected
          ? {
              create: {
                id: ID_SALES_INFO_PREFIX + createId(),
                description: itemInfo.salesInfo?.description,
                price: new Prisma.Decimal(itemInfo.salesInfo?.sellingPrice!),
                currency: "PKR",
              },
            }
          : undefined,
      },
    });

    if (createdItem) {
      logger.info("Created Item with ID:", { message: createdItem.id });
      return NextResponse.json({}, { status: 200 });
    } else {
      logger.error(
        `Failed to create item for organization ${organizationJson.id}`,
        { item: itemInfo, status: 500 }
      );
      return NextResponse.json(
        { error: { message: `Failed to create item.` } },
        { status: 500 }
      );
    }
  } catch (e) {
    logger.error(e);

    return NextResponse.json(
      { error: { message: "Server Error" } },
      { status: 500 }
    );
  }
});

const DELETE = withApiAuthRequired(async (req: NextRequest) => {
  const newHeaders = new Headers(req.headers);
  newHeaders.set("Authorization", `BEARER ${process.env.INTERNAL_SECRET}`);
  newHeaders.delete("content-length");

  const body = await req.json();

  try {
    logger.verbose(`DELETE /api/items ${JSON.stringify(req)}`, {
      itemIds: body,
    });
    const items = await prisma.item.deleteMany({ where: { id: { in: body } } });
    logger.info("Deleted item count: ", {
      message: JSON.stringify({
        count_deleted: items.count,
        item_payload: body["items"],
      }),
    });

    return NextResponse.json(items.count, { status: 200 });
  } catch (e) {
    logger.error(e);

    return NextResponse.json(
      { error: { message: "Server Error" } },
      { status: 500 }
    );
  }
});

export { POST, DELETE };
