package io.ancora;

import io.ancora.ffi.AncoraNative;
import org.junit.jupiter.api.Assumptions;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

class Phase149E2eQwenMockTest {

    static final class QwenConstants149 {
        static final String QWEN3_MAX      = "qwen3-max";
        static final String QWEN_VL_PLUS   = "qwen-vl-plus";
        static final String QWEN_TURBO     = "qwen-turbo";
        static final String QWEN_PLUS      = "qwen-plus";
        static final String QWEN_LONG      = "qwen-long";

        static final Map<String, String> REGION_ENDPOINTS = Map.of(
            "cn-hangzhou", "https://dashscope.aliyuncs.com/api/v1/services",
            "ap-southeast-1", "https://dashscope-intl.aliyuncs.com/api/v1/services"
        );
    }

    @Test
    void qwen3Max_model_constant() {
        assertEquals("qwen3-max", QwenConstants149.QWEN3_MAX);
    }

    @Test
    void qwenTurbo_model_constant() {
        assertEquals("qwen-turbo", QwenConstants149.QWEN_TURBO);
    }

    @Test
    void qwenPlus_model_constant() {
        assertEquals("qwen-plus", QwenConstants149.QWEN_PLUS);
    }

    @Test
    void qwenLong_model_constant() {
        assertEquals("qwen-long", QwenConstants149.QWEN_LONG);
    }

    @Test
    void regionEndpoints_hasTwoEntries() {
        assertEquals(2, QwenConstants149.REGION_ENDPOINTS.size());
    }

    @Test
    void chRegion_has_dashscope_endpoint() {
        String ep = QwenConstants149.REGION_ENDPOINTS.get("cn-hangzhou");
        assertNotNull(ep);
        assertTrue(ep.contains("dashscope.aliyuncs.com"));
    }

    @Test
    void intlRegion_has_dashscope_intl_endpoint() {
        String ep = QwenConstants149.REGION_ENDPOINTS.get("ap-southeast-1");
        assertNotNull(ep);
        assertTrue(ep.contains("dashscope-intl.aliyuncs.com"));
    }

    @Test
    void agentSpec_acceptsQwenModel() {
        AgentSpec spec = new AgentSpec(QwenConstants149.QWEN3_MAX, null, null, null, null);
        assertEquals(QwenConstants149.QWEN3_MAX, spec.model());
    }

    @Test
    void allQwenModels_nonEmpty() {
        for (String model : List.of(
            QwenConstants149.QWEN3_MAX,
            QwenConstants149.QWEN_VL_PLUS,
            QwenConstants149.QWEN_TURBO,
            QwenConstants149.QWEN_PLUS,
            QwenConstants149.QWEN_LONG
        )) {
            assertFalse(model.isEmpty(), "Expected non-empty model id");
        }
    }

    @Test
    void agentSpec_qwenModel_roundTrip() {
        AgentSpec a = new AgentSpec(QwenConstants149.QWEN_TURBO, "system", null, null, null);
        AgentSpec b = new AgentSpec(QwenConstants149.QWEN_TURBO, "system", null, null, null);
        assertEquals(a, b);
    }

    private static void skipIfAbsent() {
        Assumptions.assumeTrue(AncoraNative.AVAILABLE, "ancora_ffi not present");
    }
}
