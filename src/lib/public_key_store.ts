import { storable } from "./storage";

export type PublicKeyStore = Record<string, string[]>;

export const publicKeyStore = storable<PublicKeyStore>("agePublicKeys", {});
