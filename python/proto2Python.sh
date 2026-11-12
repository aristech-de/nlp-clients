#!/bin/bash
# Get the directory of the script
SCRIPT_DIR=$(dirname "$(realpath "$0")")
echo "Compiling .proto files to Python in directory: $SCRIPT_DIR"

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

# protoc generates absolute imports between the generated modules, e.g.
#   import projects_pb2 as projects__pb2   (in .py files)
#   import projects_pb2                    (in .pyi stubs)
# Turn them into package-relative imports so they work inside aristech_nlp_client.proto:
#   from . import projects_pb2 as projects__pb2
#   from . import projects_pb2
sed -i '' -E 's/^import ([A-Za-z0-9_]+_pb2)( as [A-Za-z0-9_]+)?$/from . import \1\2/' ${OUT_DIR}/*.py ${OUT_DIR}/*.pyi

# Copy the __init__.py file to the generated directory
touch ${OUT_DIR}/__init__.py
