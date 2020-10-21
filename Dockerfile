FROM openjdk:8

COPY ./data/dynamodb_local_latest.tar.gz local-data/

RUN ["tar", "-xvf", "./local-data/dynamodb_local_latest.tar.gz"]

VOLUME /data

EXPOSE 8000

CMD ["java", "-Djava.library.path=./local-data/DynamoDBLocal_lib", "-jar", "DynamoDBLocal.jar", "-dbPath", "./data"]