import * as fflate from 'fflate';

const encode_string = (string: string) => {
  const encoder = new TextEncoder();
  const bytes = encoder.encode(string);

  return bytes
}

const decode_string = (bytes: Uint8Array) => {
  const decoder = new TextDecoder('utf-8');
  const string = decoder.decode(bytes);

  return string;
}

const u16ToLeBytes = (value: number): Uint8Array => {
  const buffer = new ArrayBuffer(2);
  new DataView(buffer).setUint16(0, value, true);
  return new Uint8Array(buffer);
}

const u32ToLeBytes = (value: number): Uint8Array => {
  const buffer = new ArrayBuffer(4);
  new DataView(buffer).setUint32(0, value, true);

  return new Uint8Array(buffer);
}

const leBytesToU16 = (bytes: Uint8Array): number => {
  return new DataView(bytes.buffer).getUint16(0, true);
}

const leBytesToU32 = (bytes: Uint8Array): number => {
  return new DataView(bytes.buffer).getUint32(0, true);
}

const concat = (arrays: Uint8Array[]) => {
  let length = 0;

  for (const array of arrays) {
    length += array.length;
  }

  const buffer = new Uint8Array(length);

  let offset = 0;

  for (const array of arrays) {
    buffer.set(array, offset);
    offset += array.length;
  }

  return buffer;
}

const write = (name: string, content: Uint8Array) => {
  const name_bytes = encode_string(name);
  const name_size = u16ToLeBytes(name_bytes.length);

  const encoded_bytes = fflate.deflateSync(content);
  const encoded_size = u32ToLeBytes(encoded_bytes.length);

  return concat([
    name_size,
    name_bytes,
    encoded_size,
    encoded_bytes
  ])
}

type ArchiveEntry = {
  name: string,
  content: Uint8Array | string
}

const archive = (entries: ArchiveEntry[]) => {
  return concat(entries.map((entry) => write(entry.name, typeof entry.content === 'string' ? encode_string(entry.content) : entry.content)))
}

type File = {
  name: string,
  content: Uint8Array
}

const parse = (bytes: Uint8Array) => {
  const files: File[] = [];

  const size = bytes.length;
  let remained = size;
  let offset = 0;

  while (remained > 0) {
    const len = leBytesToU16(bytes.slice(offset, offset + 3));

    offset += 2;
    remained -= 2;

    const str = bytes.subarray(offset, offset + len);

    offset += len;
    remained -= len;

    const name = decode_string(str);

    const fileLength = leBytesToU32(bytes.slice(offset, offset + 5));

    offset += 4;
    remained -= 4;

    const file = bytes.subarray(offset, offset + fileLength);

    offset += fileLength;
    remained -= fileLength;

    const content = fflate.decompressSync(file);

    files.push({
      name,
      content
    })
  }

  return files;
}

export {
  archive,
  parse
}