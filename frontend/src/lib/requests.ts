import { token } from './stores';

const base_url = 'http://0.0.0.0:6060';

type Method = 'GET' | 'POST';
type ContentType = 'application/json' | 'void';
type ResponseType = 'json' | 'void';

enum AuthLevel {
  None,
  User,
  Admin
}

let local_token: Token | undefined = undefined;

token.subscribe((token) => (local_token = token));

function get_token(auth: AuthLevel): string | undefined {
  if (auth === AuthLevel.None) {
    return '';
  } else if (auth === AuthLevel.User && local_token !== undefined) {
    return local_token.token;
  } else if (auth === AuthLevel.Admin && local_token !== undefined && local_token.admin) {
    return local_token.token;
  }
  return undefined;
}

async function parse_response<O>(response: Response, type: ResponseType): Promise<O> {
  if (response.status < 200 || response.status > 299) {
    throw new Error()
  }

  if (type === 'json') {
    return await response.json();
  } else if (type === 'void') {
    // @ts-ignore
    return;
  }
  // @ts-ignore
  return;
}

function create_request<I, O>(
  method: Method,
  url: string,
  auth: AuthLevel = AuthLevel.User,
  type: ContentType = 'application/json',
  response_type: ResponseType = 'json'
): (input: I) => Promise<O> {
  return async (input) => {
    let options: RequestInit = { method };
    const token = get_token(auth)

    if (token === undefined) {
      throw new Error();
    }

    if (auth !== AuthLevel.None) {
      options.headers = { Authorization: `Bearer ${token}` };
    }

    if (type === 'application/json') {
      options.headers = { ...options.headers, 'Content-Type': type };
      options.body = JSON.stringify(input);
    }

    return await fetch(`${base_url}/${url}`, options).then((response) =>
      parse_response(response, response_type)
    );
  };
}

export interface CredentialModel {
  email: string;
  password: string;
}

export interface Token {
  token: string;
  expiration: Date;
  admin: boolean;
}

export const login = create_request<CredentialModel, Token>('POST', 'users/login', AuthLevel.None);
export const register = create_request<CredentialModel, void>(
  'POST',
  'users',
  AuthLevel.None,
  'application/json',
  'void'
);
export const logout = create_request<void, void>(
  'POST',
  'users/logout',
  AuthLevel.User,
  'void',
  'void'
);
