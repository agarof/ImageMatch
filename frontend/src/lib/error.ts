export enum ErrorKind { }

export interface Error {
  kind: ErrorKind,
  details?: string,
}
