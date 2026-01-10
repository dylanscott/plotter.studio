export type Length<Unit extends string> = number & { __unit: Unit };
