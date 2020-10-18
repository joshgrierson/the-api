# constants
endpoint="http://localhost:8000"
tableName="RustAPI"

echo "Creating dynamodb table..."

# aws commands
# aws dynamodb create-table \
#     --table-name $tableName \
#     --attribute-definitions \
#         AttributeName=Id,AttributeType=S \
#     --key-schema \
#         AttributeName=Id,KeyType=HASH \
#     --provisioned-throughput \
#         ReadCapacityUnits=1,WriteCapacityUnits=1 \
#     --endpoint-url $endpoint

# echo "Putting data..."

json=""

while read line;
    #json="$json\n$line"
    echo $line;
do < ./data/sample.json

# aws dynamodb put-item \
#     --table-name $tableName \
#     --item $json \
#     --endpoint-url $endpoint
