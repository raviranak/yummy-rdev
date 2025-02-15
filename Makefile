MAKE_HELP_LEFT_COLUMN_WIDTH:=14
.PHONY: help build
help: ## This help.
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-$(MAKE_HELP_LEFT_COLUMN_WIDTH)s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST) | sort

format: ## Format all the code using isort and black
	isort feast_pyspark/
	black --target-version py37 feast_pyspark

lint: ## Run mypy, isort, flake8, and black
	mypy feast_pyspark/
	isort feast_pyspark/ --check-only
	flake8 feast_pyspark/
	black --check feast_pyspark

build: ## Build the wheel
	rm -rf dist/*
	python setup.py sdist
#	python -m build

publish-testpypi: ## Publish to testpipy
	twine upload --repository testpypi dist/*

publish-pypi: ## Publish to pipy
	twine upload --repository pypi dist/*

test-polars: ## Run tests for polars backend
	FEAST_USAGE=False IS_TEST=True python3 ./tests/run.py --polars

test-dask: ## Run tests for dask backend
	FEAST_USAGE=False IS_TEST=True python3 ./tests/run.py --dask

test-ray: ## Run tests for ray backend
	FEAST_USAGE=False IS_TEST=True python3 ./tests/run.py --ray

test-nospark: 
	FEAST_USAGE=False IS_TEST=True python3 ./tests/run.py --nospark

test-spark: ## Run tests for spark backend
	PYSPARK_PYTHON=/opt/conda/bin/python3 PYSPARK_DRIVER_PYTHON=/opt/conda/bin/python3 FEAST_USAGE=False IS_TEST=True spark-submit \
				   --packages io.delta:delta-core_2.12:1.1.0 \
				   --conf "spark.sql.extensions=io.delta.sql.DeltaSparkSessionExtension" \
				   --conf "spark.sql.catalog.spark_catalog=org.apache.spark.sql.delta.catalog.DeltaCatalog" \
				   --conf "spark.driver.memory=5g" \
				   --conf "spark.executor.memory=5g" \
				   ./tests/run.py --spark

test-delta:
	PYSPARK_PYTHON=/opt/conda/bin/python3 PYSPARK_DRIVER_PYTHON=/opt/conda/bin/python3 FEAST_USAGE=False IS_TEST=True spark-submit \
				   --packages io.delta:delta-core_2.12:1.1.0 \
				   --conf "spark.sql.extensions=io.delta.sql.DeltaSparkSessionExtension" \
				   --conf "spark.sql.catalog.spark_catalog=org.apache.spark.sql.delta.catalog.DeltaCatalog" \
				   --conf "spark.driver.memory=5g" \
				   --conf "spark.executor.memory=5g" \
				   ./tests/run.py --delta

test-iceberg:
	PYSPARK_PYTHON=/opt/conda/bin/python3 PYSPARK_DRIVER_PYTHON=/opt/conda/bin/python3 FEAST_USAGE=False IS_TEST=True spark-submit \
				   --packages io.delta:delta-core_2.12:1.1.0 \
				   --packages org.apache.iceberg:iceberg-spark-runtime-3.2_2.12:0.13.2 \
				   --conf "spark.sql.extensions=io.delta.sql.DeltaSparkSessionExtension" \
				   --conf "spark.sql.catalog.spark_catalog=org.apache.spark.sql.delta.catalog.DeltaCatalog" \
				   --conf "spark.driver.memory=5g" \
				   --conf "spark.executor.memory=5g" \
				   ./tests/run.py --iceberg
