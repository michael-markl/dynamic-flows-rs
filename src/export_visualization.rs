use crate::{
    dynamic_flow::{DynamicFlow, FlowRatesCollection},
    num::Num,
    piecewise_constant::PiecewiseConstant,
    piecewise_linear::PiecewiseLinear,
};
use serde::{
    ser::{SerializeMap, SerializeStruct},
    Serialize, Serializer,
};

struct JsonNumber(f64);

impl Serialize for JsonNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.0.is_nan() {
            serializer.serialize_str("NaN")
        } else if self.0.is_infinite() {
            if self.0.is_sign_positive() {
                serializer.serialize_str("Infinity")
            } else {
                serializer.serialize_str("-Infinity")
            }
        } else {
            serializer.serialize_f64(self.0)
        }
    }
}

struct SerializableIterator<I: Serialize, T: Iterator<Item = I>>(T);

impl<I: Serialize, T: Iterator<Item = I> + Clone> Serialize for SerializableIterator<I, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_seq(self.0.clone())
    }
}

pub struct VisualizationPiecewiseLinear<'a, T: Num>(&'a PiecewiseLinear<T>);

impl<'a, T: Num> Serialize for VisualizationPiecewiseLinear<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut res = serializer.serialize_struct("PiecewiseLinear", 5)?;
        res.serialize_field(
            "times",
            &SerializableIterator(self.0.points().iter().map(|p| JsonNumber(p.0.to_f64()))),
        )?;
        res.serialize_field(
            "values",
            &SerializableIterator(self.0.points().iter().map(|p| JsonNumber(p.1.to_f64()))),
        )?;
        res.serialize_field("firstSlope", &JsonNumber(self.0.first_slope().to_f64()))?;
        res.serialize_field("lastSlope", &JsonNumber(self.0.last_slope().to_f64()))?;
        res.serialize_field("domain", &self.0.domain().map(|x| JsonNumber(x.to_f64())))?;
        res.end()
    }
}

pub struct VisualizationPiecewiseConstant<'a, T: Num>(&'a PiecewiseConstant<T>);

impl<'a, T: Num> Serialize for VisualizationPiecewiseConstant<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut res = serializer.serialize_struct("PiecewiseConstant", 5)?;
        res.serialize_field(
            "times",
            &SerializableIterator(self.0.points().iter().map(|p| JsonNumber(p.0.to_f64()))),
        )?;
        res.serialize_field(
            "values",
            &SerializableIterator(self.0.points().iter().map(|p| JsonNumber(p.1.to_f64()))),
        )?;
        res.serialize_field("domain", &self.0.domain().map(|x| JsonNumber(x.to_f64())))?;
        res.end()
    }
}

pub struct VisualizationDynamicFlow<'a, T: Num>(&'a DynamicFlow<T>);

impl<'a, T: Num> Serialize for VisualizationDynamicFlow<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut res = serializer.serialize_struct("DynamicFlow", 2)?;
        res.serialize_field(
            "queues",
            &SerializableIterator(
                self.0
                    .queues()
                    .iter()
                    .map(|q| VisualizationPiecewiseLinear(q)),
            ),
        )?;
        res.serialize_field(
            "inflow",
            &SerializableIterator(self.0.inflow().iter().map(|f| VisualizationFlowRates(f))),
        )?;
        res.serialize_field(
            "outflow",
            &SerializableIterator(self.0.outflow().iter().map(|f| VisualizationFlowRates(f))),
        )?;
        res.end()
    }
}

pub struct VisualizationFlowRates<'a, T: Num>(&'a FlowRatesCollection<T>);

impl<'a, T: Num> Serialize for VisualizationFlowRates<'a, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut res: <S as Serializer>::SerializeMap =
            serializer.serialize_map(Some(self.0.function_by_comm().len()))?;
        for (comm, f) in self.0.function_by_comm() {
            res.serialize_entry(comm, &VisualizationPiecewiseConstant(f))?;
        }
        res.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        float::F64,
        network_loader::{NetworkLoader, PathInflow},
        num::Num,
        piecewise_constant::PiecewiseConstant,
        points,
    };

    use super::VisualizationDynamicFlow;

    #[test]
    pub fn test_serialization_to_json() {
        let network_loader: NetworkLoader<F64> = NetworkLoader::new(&[
            PathInflow {
                path: &[0, 1, 2],
                inflow: &PiecewiseConstant::new(
                    [-F64::INFINITY, F64::INFINITY],
                    points![(0.0, 1.0), (3.0, 0.0)],
                ),
            },
            PathInflow {
                path: &[2, 0, 1],
                inflow: &PiecewiseConstant::new(
                    [-F64::INFINITY, F64::INFINITY],
                    points![(0.0, 2.0), (3.0, 0.0)],
                ),
            },
        ]);
        let flow = network_loader.build_flow(
            3,
            &[1.0.into(), 2.0.into(), 3.0.into()],
            &[(1.0 / 1.0).into(), (1.0 / 2.0).into(), (1.0 / 3.0).into()],
            &[1.0.into(), 2.0.into(), 3.0.into()],
        );
        let result = serde_json::to_string_pretty(&VisualizationDynamicFlow(&flow)).unwrap();
        println!("{}", result)
    }
}
