// src/tests.rs
mod district_processor;

#[cfg(test)]
mod tests {
    use super::district_processor::process_districts;
    use std::fs;

    #[test]
    fn test_district_processing() {
        // 备份原始数据
        let original_data = fs::read_to_string("exercises/hard/solutiont3/district.json")
            .expect("Failed to read district.json");
        
        // 运行处理逻辑
        process_districts();

        // 验证输出结果
        // (需要根据实际数据断言输出结果)
        // assert_eq!(output, "3,2");
        
        // 恢复原始数据（如果测试修改了文件）
        fs::write("exercises/hard/solutiont3/district.json", original_data)
            .expect("Failed to restore district.json");
    }
}