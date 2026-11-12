use bigdecimal::BigDecimal;
use uuid::Uuid;

struct DetalleVenta {
    id: Uuid,
    concepto: String,
    descripcion: String,
    tipo: ContenidoVenta,
    cantidad: u32,
    precio: BigDecimal,
    subtotal: BigDecimal,
}

enum ContenidoVenta {
    Servicio,
    Paquete,
}
