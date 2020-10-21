# constants
endpoint="http://localhost:8000"
tableName="RustAPI"

echo "Creating dynamodb table..."

aws dynamodb create-table \
    --table-name $tableName \
    --attribute-definitions \
        AttributeName=Id,AttributeType=S \
    --key-schema \
        AttributeName=Id,KeyType=HASH \
    --provisioned-throughput \
        ReadCapacityUnits=1,WriteCapacityUnits=1 \
    --endpoint-url $endpoint

echo "Putting data..."

json=""

for line in $(cat ./data/sample.json); do
    json="$json$line"
done

aws dynamodb put-item \
    --table-name $tableName \
    --item $json \
    --endpoint-url $endpoint