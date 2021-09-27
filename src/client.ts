import { Connection, Keypair } from "@solana/web3.js";

import path from 'path';
import os from 'os';
import fs from 'mz/fs';
import yaml from 'yaml';

export async function connection() :Promise<void> {
    try{
    const CONFIG_FILE_PATH = path.resolve(
        os.homedir(),
        '.config',
        'solana',
        'cli',
        'config.yml',
      );
      const configYml = await fs.readFile(CONFIG_FILE_PATH, {encoding: 'utf8'});
      const config=yaml.parse(configYml);
    const rpcUrl= config.json_rpc_url;
    const connection:Connection =new Connection(rpcUrl,"confirmed");
    const version=await connection.getVersion();
    console.log("Connected to cluster working -----Cluster type--"+rpcUrl,"Cluster version--"+version);

    } catch (err) {
        console.log("Error while connecting ,Message-"+err.toString());
    }


   
    
}


/*

curl http://localhost:8899 -X POST -H "Content-Type: application/json" -d '
  {
    "jsonrpc": "2.0",
    "id": 1,
    "method": "getBalance",
    "params": [
      "8hmenUPekm8Gr735tt9Hq65YhJggXsAF8hYPJVx2qnaW"
    ]
  }
'

*/


