#!/bin/bash
SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)

# This script converts the protocol buffer files to python
OUT_DIR="$SCRIPT_DIR/src/aristech_nlp_client/proto"

# Directory containing the proto files
PROTO_DIR="$SCRIPT_DIR/../protos"

rm -rf ${OUT_DIR}
mkdir -p ${OUT_DIR}

# Generate python from proto files using grpcio-tools
python -m grpc_tools.protoc \
  -I ${PROTO_DIR} \
  --proto_path=${PROTO_DIR} \
  --python_out=${OUT_DIR} \
  --grpc_python_out=${OUT_DIR} \
  --plugin=protoc-gen-mypy=$(which protoc-gen-mypy) \
  --mypy_out=${OUT_DIR} \
  ${PROTO_DIR}/*.proto

# Replace these import paths in the generated files
# import nlp_server_pb2 as nlp__server__pb2
# import projects_pb2 as projects__pb2
# import intents_pb2 as intents__pb2
# with
# from . import nlp_server_pb2 as nlp__server__pb2
# from . import projects_pb2 as projects__pb2
# from . import intents_pb2 as intents__pb2
sed -i '' 's/import nlp_server_pb2 as nlp__server__pb2/from . import nlp_server_pb2 as nlp__server__pb2/g' ${OUT_DIR}/nlp_server_pb2_grpc.py
sed -i '' 's/import projects_pb2 as projects__pb2/from . import projects_pb2 as projects__pb2/g' ${OUT_DIR}/projects_pb2_grpc.py
sed -i '' 's/import intents_pb2 as intents__pb2/from . import intents_pb2 as intents__pb2/g' ${OUT_DIR}/intents_pb2_grpc.py

# Copy the __init__.py file to the generated directory
touch ${OUT_DIR}/__init__.py
