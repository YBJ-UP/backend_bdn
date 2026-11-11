use uuid::Uuid;

use crate::model::area::Department;

enum TipoMovimiento {
    Compra,
    Venta,
}

struct CatalogoMovimientos {
    nombre: String,
    tipo: TipoMovimiento,
}

struct MovimientoInterni {
    id: Uuid,
    area: Department,
    movimiento: CatalogoMovimientos,
    costo: f64,
    motivo: String,
    fecha: String,
}
