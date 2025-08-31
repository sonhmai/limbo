issue: https://github.com/tursodatabase/turso/issues/2066

todo
- [ ] include ext as workspace dependency
- [ ] add ext as feature in core/Cargo.toml
- [ ] register ext in core


## Implementation Plan

### Phase 1: Core Functions
1. **Project Setup**
   - Add `h3o` dependency to Cargo.toml
   - Create extension module structure
   - Set up basic registration framework

2. **Basic Scalar Functions**
   - Implement coordinate conversion functions
   - Add cell property functions (resolution, validity)
   - Basic neighbor and hierarchy functions

### Phase 2: Advanced Functions
3. **Spatial Analysis Functions**
   - Distance calculations
   - K-ring operations
   - Boundary and polygon operations

4. **Optimization**
   - Efficient bulk operations
   - Memory optimization for large datasets

### Phase 3: Virtual Table Integration
5. **Virtual Table Implementation**
   - H3 spatial indexing virtual table
   - Query optimization for spatial operations
   - Integration with Turso's query planner

6. **Performance Optimization**
   - Index structures for H3 data
   - Query optimization patterns
   - Caching strategies

### Phase 4: Testing and Documentation
7. **Comprehensive Testing**
   - Unit tests for all functions
   - Integration tests with real geospatial data
   - Performance benchmarks

## Future Enhancements

### Additional Functions
- Polygon to H3 cells conversion
- Line intersection with H3 cells
- Spatial aggregation functions
- Time-based spatial indexing

### Integration Features
- GeoJSON import/export
- Integration with mapping libraries
- Visualization helpers

### Performance Optimizations
- SIMD operations for bulk conversions
- Custom index structures
- Query result caching