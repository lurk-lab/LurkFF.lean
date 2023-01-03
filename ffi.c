#include <lean/lean.h>

#define intern inline static
#define l_arg b_lean_obj_arg
#define l_res lean_obj_res 

intern l_res mk_byte_array_from(size_t len) {
    lean_sarray_object* o = (lean_sarray_object*)lean_alloc_object(
        sizeof(lean_sarray_object) + len
    );
    lean_set_st_header((lean_object*)o, LeanScalarArray, 1);
    o->m_size = len;
    o->m_capacity = len;
    return o;
}

intern uint64_t* lean_byte_array_to_uint64(l_arg a) {
    return (uint64_t*)lean_to_sarray(a)->m_data;
}

// This :: ByteArray -> ByteArray -> ByteArray -> ByteArray -> ByteArray

extern l_res lean_poseidon_hash_arity_4 (l_arg a, l_arg b, l_arg c, l_arg d) {
    lean_sarray_object* answer = mk_byte_array_from(32); 

    uint64_t* a1 = lean_byte_array_to_uint64(a);
    uint64_t* b1 = lean_byte_array_to_uint64(b);
    uint64_t* c1 = lean_byte_array_to_uint64(c);
    uint64_t* d1 = lean_byte_array_to_uint64(d);

    hash_arity_4(answer->m_data, a1, b1, c1, d1);

    return answer;
}