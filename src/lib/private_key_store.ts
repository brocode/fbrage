import { storable } from "./storage";

export type PrivateKeyStore = Record<string, string>;

export const privateKeyStore = storable<PrivateKeyStore>("agePrivateKeys", {});
